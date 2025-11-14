//! A simple `Dataclass` derive macro inspired by Python's `dataclasses`.
//!
//! Features implemented:
//! * Generate `pub fn new(...) -> Self` constructor which accepts values for non-default fields
//! * Support field-level defaults via `#[dataclass(default)]` and `#[dataclass(default = "expr")]` (expr as string literal)
//! * Implement `Clone`, `Debug`, `PartialEq`, `Eq` for the struct
//! * Implement `Default` when all fields have defaults
//!
//! Examples:
//!
//! ```rust
//! use dataclasses::Dataclass;
//!
//! #[derive(Dataclass)]
//! struct Person {
//!     name: String,
//!     age: i32,
//!     #[dataclass(default)]
//!     nickname: Option<String>,
//!     #[dataclass(default = "Vec::new()")]
//!     tags: Vec<String>,
//! }
//!
//! let p = Person::new("Alice".into(), 30);
//! assert_eq!(p.nickname, None);
//! ```
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Meta, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(Dataclass, attributes(dataclass))]
pub fn dataclass_macro(input: TokenStream) -> TokenStream {
    // Parse input
    let input = parse_macro_input!(input as DeriveInput);

    match impl_dataclass(&input) {
        Ok(ts) => ts.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn impl_dataclass(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Only support structs with named fields
    let fields = match &input.data {
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Named(named) => &named.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "Dataclass macro only supports structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "Dataclass macro requires a struct",
            ));
        }
    };

    // For each field collect info
    struct FieldInfo {
        ident: syn::Ident,
        ty: syn::Type,
        default: Option<proc_macro2::TokenStream>,
    }

    let mut infos = Vec::new();
    for field in fields.iter() {
        let ident = field
            .ident
            .clone()
            .expect("named fields should have idents");
        let ty = field.ty.clone();

        let mut default = None;
        for attr in &field.attrs {
            if attr.path().is_ident("dataclass") {
                // attr like #[dataclass(default)] or #[dataclass(default = "Vec::new()")]
                // For syn 2: the meta tokens are in attr.meta or Meta::List(meta)
                if let Meta::List(list) = &attr.meta {
                    let tokens = list.tokens.to_string();
                    // tokens look like "(default)" or "(default = \"Vec::new()\")"
                    let inside = tokens.trim();
                    let inside = inside.trim_start_matches('(').trim_end_matches(')');
                    // Split on commas, but ignore commas inside double quotes
                    let mut parts = Vec::new();
                    let mut start = 0usize;
                    let mut in_quotes = false;
                    for (i, c) in inside.char_indices() {
                        match c {
                            '"' => in_quotes = !in_quotes,
                            ',' if !in_quotes => {
                                parts.push(inside[start..i].trim());
                                start = i + 1;
                            }
                            _ => {}
                        }
                    }
                    if start < inside.len() {
                        parts.push(inside[start..].trim());
                    }
                    for part in parts.into_iter().filter(|s| !s.is_empty()) {
                        if part == "default" {
                            default = Some(quote! { ::core::default::Default::default() });
                        } else if part.starts_with("default=") || part.starts_with("default =") {
                            // find the literal string after '='
                            if let Some(eq_idx) = part.find('=') {
                                let rhs = part[eq_idx + 1..].trim();
                                // strip possible surrounding quotes
                                let rhs = if rhs.starts_with('"') && rhs.ends_with('"') {
                                    &rhs[1..rhs.len() - 1]
                                } else {
                                    rhs
                                };
                                let expr: syn::Expr = syn::parse_str(rhs).map_err(|e| {
                                    syn::Error::new(
                                        field.span(),
                                        format!("invalid default expression: {}", e),
                                    )
                                })?;
                                default = Some(quote! { #expr });
                            }
                        } else {
                            return Err(syn::Error::new(
                                field.span(),
                                "unknown dataclass attribute",
                            ));
                        }
                    }
                }
            }
        }

        infos.push(FieldInfo { ident, ty, default });
    }

    // Build 'new' function params and body
    let mut params = Vec::new();
    let mut construct_fields = Vec::new();
    let mut all_have_default = true;
    for info in &infos {
        let ident = &info.ident;
        let ty = &info.ty;
        if info.default.is_none() {
            params.push(quote! { #ident: #ty });
            construct_fields.push(quote! { #ident });
            all_have_default = false;
        } else {
            let expr = info.default.as_ref().unwrap();
            construct_fields.push(quote! { #ident: #expr });
        }
    }

    // Collect clones for clone impl and fields for Debug/PartialEq
    let field_idents: Vec<_> = infos.iter().map(|f| f.ident.clone()).collect();
    let field_idents_ref: Vec<_> = field_idents.iter().collect();

    // Determine generics type params for where clauses
    let type_idents: Vec<syn::Ident> = generics
        .params
        .iter()
        .filter_map(|p| match p {
            syn::GenericParam::Type(ty) => Some(ty.ident.clone()),
            _ => None,
        })
        .collect();

    let mut clone_bounds = where_clause.cloned();
    let mut debug_bounds = where_clause.cloned();
    let mut partial_bounds = where_clause.cloned();
    let mut eq_bounds = where_clause.cloned();
    let mut default_bounds = where_clause.cloned();

    if !type_idents.is_empty() {
        let bounds_tokens =
            quote! { #(#type_idents: Clone + std::fmt::Debug + PartialEq + Eq + Default),* };
        clone_bounds = Some(syn::parse2(quote! { where #bounds_tokens })?);
        debug_bounds = clone_bounds.clone();
        partial_bounds = clone_bounds.clone();
        eq_bounds = clone_bounds.clone();
        default_bounds = clone_bounds.clone();
    }

    // Build impl tokens
    let name_str = name.to_string();
    let new_fn = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn new(#(#params),*) -> Self {
                Self { #(#construct_fields),* }
            }
        }
    };

    // Clone impl
    let clone_assigns = field_idents_ref
        .iter()
        .map(|ident| quote! { #ident: self.#ident.clone() });
    let clone_impl = quote! {
        impl #impl_generics Clone for #name #ty_generics #clone_bounds {
            fn clone(&self) -> Self {
                Self { #(#clone_assigns),* }
            }
        }
    };

    // Debug impl
    let debug_fields = field_idents_ref
        .iter()
        .map(|ident| quote! { .field(stringify!(#ident), &self.#ident) });
    let debug_impl = quote! {
        impl #impl_generics std::fmt::Debug for #name #ty_generics #debug_bounds {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(#name_str)
                    #(#debug_fields)*
                    .finish()
            }
        }
    };

    // PartialEq impl
    let eq_checks = field_idents_ref
        .iter()
        .map(|ident| quote! { self.#ident == other.#ident });
    let eq_impl = quote! {
        impl #impl_generics PartialEq for #name #ty_generics #partial_bounds {
            fn eq(&self, other: &Self) -> bool {
                #(#eq_checks)&&*
            }
        }
        impl #impl_generics Eq for #name #ty_generics #eq_bounds {}
    };

    // Default impl only if all fields have a default expression
    let default_impl = if all_have_default {
        let default_assigns = infos.iter().map(|f| {
            let id = &f.ident;
            let expr = f.default.as_ref().unwrap();
            quote! { #id: #expr }
        });
        Some(quote! {
            impl #impl_generics Default for #name #ty_generics #default_bounds {
                fn default() -> Self {
                    Self { #(#default_assigns),* }
                }
            }
        })
    } else {
        None
    };

    let expanded = quote! {
        #new_fn
        #clone_impl
        #debug_impl
        #eq_impl
        #default_impl
    };

    Ok(expanded)
}

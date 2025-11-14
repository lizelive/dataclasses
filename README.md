# dataclasses - dataclass-like derive macro for Rust

A small, pragmatic `#[derive(Dataclass)]` procedural macro inspired by Python's `dataclasses` module. It generates common boilerplate for `struct` types, including constructors, `Debug`, `Clone`, `PartialEq`, `Eq`, and optional `Default` when all fields provide defaults.

This crate is intended for developers who like the ergonomics of Python dataclasses and want a lightweight, expressive Rust macro that handles common patterns. It's not an exhaustive reimplementation of Python dataclasses, but a carefully chosen set of features that map well to Rust idioms.

## Features

- Generate `pub fn new(...) -> Self` for named fields, with required fields first and fields with defaults omitted from the constructor
- Per-field defaults via attribute syntax:
  - `#[dataclass(default)]` - uses `Default::default()` for that field
  - `#[dataclass(default = "Expr")]` - uses the provided Rust expression string as the default (e.g., `#[dataclass(default = "Vec::new()")]`)
- Derive and implement `Clone`, `Debug`, `PartialEq` and `Eq` automatically for the struct
- Implement `Default` when every field has a default provided
- Works with generic structs (adds trait bounds as needed for the generated impls)

## Quick start

Add this crate as a dependency in your `Cargo.toml` and enable the `proc-macro` server if needed. This crate is a `proc-macro`, so import the derive attribute method like other derive macros.

Example usage:

```rust
use dataclasses::Dataclass;

#[derive(Dataclass)]
struct Person {
    name: String,
    age: i32,
    #[dataclass(default)]
    nickname: Option<String>,
    #[dataclass(default = "Vec::new()")]
    tags: Vec<String>,
}

let person = Person::new("Alice".to_string(), 30);
assert_eq!(person.nickname, None);
assert_eq!(person.tags.len(), 0);
```

When all fields provide defaults, the crate generates a `Default` impl too:

```rust
#[derive(Dataclass)]
struct Defaults {
    #[dataclass(default = "1")]
    a: i32,
    #[dataclass(default)]
    b: String,
}

let d: Defaults = Default::default();
assert_eq!(d.a, 1);
```

## Attributes

- `#[dataclass(default)]` - Use `Default::default()` for this field
- `#[dataclass(default = "Expr")]` - Use the provided `Expr` (a string literal) as the default

Notes:

- Use double-quoted expressions for `default` if the expression contains commas or spaces - the parser expects a single string for the expression.
- Only named struct fields are supported at present.

## Limitations and planned improvements

- Field-level fine-grained control flags like `eq=false`, `skip` for `Debug`/`PartialEq`, `init=False`, and `repr` are not yet available
- `post_init` hooks and `frozen` immutability are not implemented
- The `default = "Expr"` currently requires a string literal for the expression; future work could parse expressions directly
- Trait bound generation is conservative - we add typical bounds for generics when deriving traits; for finer control you can manually implement traits

If you want additional features, please open an issue or a PR.

## License

This crate is dual-licensed under MIT or Apache-2.0. You can select either license when using or distributing this crate.

Files: `LICENSE-MIT` and `LICENSE-APACHE` are included in this repository.

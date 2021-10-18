
type Value = serde_value::Value;


#[derive(Debug, Clone)]
pub struct Type {
    name: String,
    generics: Vec<Type>,
}


#[derive(Debug)]
pub struct Dataclass {
    name: String,
    init: bool,
    repr: bool,
    eq: bool,
    order: bool,
    hash: bool,
    frozen: bool,
    kw_only: bool,
    namespace: String,
    fields: Vec<Field>,
    post_init: Option<String>,
}

#[derive(Debug)]
pub enum ValueProvider {
    Factory(String),
    Value(Value),
}

#[derive(Debug)]
pub struct Field {
    name: String,
    r#type: Type,
    default: Option<Value>,
    default_factory: Option<String>,
    hash: Option<bool>,
    validate: Option<String>,//Option<Box<dyn Fn(Value) -> bool>>
    init: bool,
    repr: bool,
    compare: bool,
    kw_only: bool,
    metadata: Option<Value>,
    init_var: bool,
}


impl Default for Dataclass {
    fn default() -> Self {
        Self {
            name: Default::default(),
            init: true,
            repr: true,
            eq: true,
            order: false,
            hash: false,
            frozen: false,
            kw_only: Default::default(),
            namespace: Default::default(),
            fields: Vec::new(),
            post_init: None,
        }
    }
}
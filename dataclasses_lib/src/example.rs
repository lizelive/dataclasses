#[derive(Dataclass)]
#[dataclass(frozen = true, post_init = "post_init")]
struct InventoryItem {
    #[field(default = "helloworld")]
    name: String,
    
    #[field(default_factory = "killme")]
    adress: String,

    #[field(init = false)]
    generated: String,
}




/// https://docs.python.org/3/library/dataclasses.html
trait Dataclass {
    type Builder;
    fn new(builder: Builder);
    fn fields() -> Vec<Field>; // gets the fields for the type
    fn as_dict(&self) -> HashMap<String, Value>; // convert
    fn as_vec(&self) -> Vec<Value>;
    fn replace(&self, changes: Builder) -> self;
}


impl InventoryItem {
    fn init(name: String, adress: String, builder: Builder){
        
    }
    fn name()
}
//fn as_tuple(&self) -> (Value,Value,Value);

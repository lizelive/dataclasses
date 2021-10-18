use std::{collections::HashMap, default::Default, marker::PhantomData};

use serde_value::Value;
use codegen::{Scope, Field as CField};



struct ByName<T>{
    name: String,
    data: PhantomData<T>,
}

impl<T> ByName<T> {
    fn new(name: String) -> Self { Self { name, data: PhantomData } }
}









mod def;
mod gen;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use dataclasses::Dataclass;

#[test]
fn basic_new_and_defaults() {
    #[derive(Dataclass)]
    struct Person {
        name: String,
        age: i32,
        #[dataclass(default)]
        nickname: Option<String>,
        #[dataclass(default = "Vec::new()")]
        tags: Vec<String>,
    }

    let p = Person::new("Alice".to_string(), 30);
    assert_eq!(p.name, "Alice");
    assert_eq!(p.age, 30);
    assert_eq!(p.nickname, None);
    assert_eq!(p.tags.len(), 0);

    let cloned = p.clone();
    assert_eq!(p, cloned);
}

#[test]
fn default_impl_all_default() {
    #[derive(Dataclass)]
    struct AllDefault {
        #[dataclass(default = "1")]
        a: i32,
        #[dataclass(default)]
        b: String,
    }

    let d: AllDefault = Default::default();
    assert_eq!(d.a, 1);
    assert_eq!(d.b, String::new());
}

#[test]
fn generics_and_defaults() {
    #[derive(Dataclass)]
    struct Wrapper<T> {
        value: T,
        #[dataclass(default)]
        opt: Option<T>,
    }

    let w = Wrapper::new(100i32);
    assert_eq!(w.value, 100);
    assert_eq!(w.opt, None);
}

#[test]
fn default_expr_with_commas() {
    #[derive(Dataclass)]
    struct WithVec {
        #[dataclass(default = "vec![1, 2]")]
        v: Vec<i32>,
    }

    let d: WithVec = Default::default();
    assert_eq!(d.v, vec![1, 2]);
}

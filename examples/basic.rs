use struct_auto_from::auto_from;

#[derive(Clone)]
struct Model1 {
    id: i32,
    name: String,
    attrs: Vec<String>,
}

#[auto_from(Model1)]
struct Model2 {
    id: i32,
    name: String,
    attrs: Vec<String>,
}

fn main() {
    let m1: Model1 = Model1 {
        id: 0,
        name: "M".into(),
        attrs: vec![],
    };
    let m2: Model2 = m1.clone().into();

    assert_eq!(m1.id, m2.id);
    assert_eq!(m1.name, m2.name);
    assert_eq!(m1.attrs, m2.attrs);
}

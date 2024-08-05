use serde::Serialize;
use std::collections::HashMap;
use struct_auto_from::auto_from;

#[derive(Debug, Clone)]
struct Model1 {
    id: i32,
    name: String,
    attrs: Vec<String>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
struct Model1a {
    id: i32,
    nom: &'static str,
    attrs: Vec<String>,
    m: Model1,
}

#[allow(unused)]
#[auto_from(Model1)]
#[derive(Debug, Clone, Serialize, Default)]
struct Model2a {
    id: i32,
    name: String,
    attrs: Vec<String>,
    #[auto_from_attr(default_value = Default::default())]
    meta: HashMap<String, String>,
}

#[allow(unused)]
#[auto_from(Model1a, Model2a)]
#[derive(Debug, Serialize)]
struct Model3a {
    #[auto_from_attr(default_value = 0)]
    id: i32,

    #[serde(rename = "new_name")]
    #[auto_from_attr(from_field = "nom")]
    name: String,

    attrs: Vec<String>,

    #[auto_from_attr(default_value = Default::default())]
    #[serde(skip_serializing)]
    #[auto_from_attr(from_field = "meta", from_struct = "Model2a")]
    metadata: HashMap<String, String>,

    #[serde(rename = "model")]
    #[auto_from_attr(from_struct = "Model2a", default_value = Default::default())]
    m: Model2a,
}

fn main() {
    println!("Model3a from Model1a");
    {
        let model1a = Model1a {
            id: 1,
            nom: "Xyz",
            attrs: vec!["a".into(), "b".into()],
            m: Model1 {
                id: 99,
                name: "Mary".into(),
                attrs: vec!["x".into(), "y".into(), "z".into()],
            },
        };

        let model3a: Model3a = model1a.clone().into();

        println!("model1a={model1a:?}");
        println!("model3a={model3a:?}");
        println!("model3a to JSON: {:?}", serde_json::to_value(model3a));
    }

    println!();

    println!("Model3a from Model2a");
    {
        let model2a = Model2a {
            id: 1,
            name: "Xyz".into(),
            attrs: vec!["a".into(), "b".into()],
            meta: HashMap::from([("abc".into(), "111".into()), ("def".into(), "222".into())]),
        };

        let model3a: Model3a = model2a.clone().into();

        println!("model2a={model2a:?}");
        println!("model3a={model3a:?}");
        println!("model3a to JSON: {:?}", serde_json::to_value(model3a));
    }
}

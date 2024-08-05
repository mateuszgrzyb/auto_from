use fake::{Dummy, Fake, Faker};
use serde::Serialize;
use serde_json::{self, Number, Value};
use std::collections::HashMap;
use struct_auto_from::auto_from;

#[auto_from(Model2a)]
#[derive(PartialEq, Eq, Debug, Clone, Dummy, Serialize)]
struct Model1 {
    id: i32,
    name: String,
    attrs: Vec<String>,
}

#[auto_from(Model3a)]
#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
struct Model1a {
    id: i32,

    #[auto_from_attr(default_value = "\"XYZ\"")]
    nom: &'static str,

    attrs: Vec<String>,
    m: Model1,
}

#[auto_from(Model1)]
#[derive(PartialEq, Eq, Debug, Clone, Dummy, Serialize, Default)]
struct Model2a {
    id: i32,
    name: String,
    attrs: Vec<String>,
    #[auto_from_attr(default_value = Default::default())]
    meta: HashMap<String, String>,
}

#[auto_from(Model1a, Model2a)]
#[derive(PartialEq, Eq, Debug, Clone, Dummy, Serialize, Default)]
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

#[test]
fn test_auto_from_rename_and_multiple_from_model_1() {
    {
        // given
        let m1a = Model1a {
            id: 1,
            nom: "Joe",
            attrs: Faker.fake(),
            m: Faker.fake(),
        };
        let m1a_j = serde_json::to_value(m1a.clone()).unwrap();

        // when
        let m1a_3a: Model3a = m1a.clone().into();
        let m1a_3a_j = serde_json::to_value(m1a_3a.clone()).unwrap();

        // then

        let m1a_3a_j_exp = {
            let mut m1a_j_x = m1a_j.clone();

            let Value::Object(ref mut map) = m1a_j_x else {
                unreachable!()
            };

            {
                map.insert("id".into(), Value::Number(Number::from(0)));
            }

            {
                let value = map.remove("nom").unwrap();
                map.insert("new_name".into(), value);
            }

            {
                let mut value = map.remove("m").unwrap();
                {
                    let Value::Object(ref mut map) = value else {
                        unreachable!()
                    };
                    map.insert("meta".into(), Value::Object(Default::default()));
                }
                map.insert("model".into(), value);
            }

            m1a_j_x
        };

        assert_eq!(m1a_3a.id, 0);
        assert_eq!(m1a_3a.name, m1a.nom.to_owned());
        assert_eq!(m1a_3a.attrs, m1a.attrs);
        assert_eq!(m1a_3a.metadata, Default::default());
        assert_eq!(m1a_3a.m, m1a.m.into());
        assert_eq!(m1a_3a_j, m1a_3a_j_exp);
    }
}

#[test]
fn test_auto_from_rename_and_multiple_from_model_2() {
    {
        // given
        let m2a = Model2a {
            id: 1,
            name: Faker.fake(),
            attrs: Faker.fake(),
            meta: Faker.fake(),
        };

        // when
        let m2a_3a: Model3a = m2a.clone().into();

        // then
        assert_eq!(m2a_3a.id, m2a.id);
        assert_eq!(m2a_3a.name, m2a.name);
        assert_eq!(m2a_3a.attrs, m2a.attrs);
        assert_eq!(m2a_3a.metadata, m2a.meta);
        assert_eq!(m2a_3a.m, Default::default());
    }
}

#[test]
fn test_auto_from_loop() {
    {
        // given

        let m1a = Model1a {
            id: 1,
            nom: "Joe",
            attrs: Faker.fake(),
            m: Faker.fake(),
        };

        let m1a_3a: Model3a = m1a.clone().into();

        // when
        let m1a_3a_1a: Model1a = m1a_3a.into();

        // then
        assert_eq!(m1a_3a_1a.id, 0);
        assert_eq!(m1a_3a_1a.nom, "XYZ");
        assert_eq!(m1a_3a_1a.attrs, m1a.attrs);
        assert_eq!(m1a_3a_1a.m, m1a.m);
    }
}

use std::collections::HashMap;
use struct_auto_from::auto_from;

pub struct UserModel {
    #[allow(unused)]
    id: i32,
    nom: String,
    email: String,
}

#[auto_from(UserModel)]
#[derive(Debug)]
pub struct UserType {
    #[auto_from_attr(default_value = 42)]
    id: i32,
    #[auto_from_attr(from_field = "nom")]
    name: String,
    #[auto_from_attr(default_value = Default::default())]
    metadata: HashMap<String, String>,
    email: String,
}

fn main() {
    let user_model = UserModel {
        id: 1234,
        nom: "GvR".into(),
        email: "me@example.com".into(),
    };

    let user_type: UserType = user_model.into();

    println!("{user_type:?}");

    assert_eq!(user_type.id, 42);
    assert_eq!(user_type.name, "GvR");
    assert_eq!(user_type.email, "me@example.com");
    assert_eq!(user_type.metadata, HashMap::new());
}

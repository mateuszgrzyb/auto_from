# Struct Auto From

Simple Rust library for auto generating conversion methods between structs.

When specifying a conversion, each field in the receiver must either be defined in the sender,
or specify a corresponding field with a different name in the sender,
or have its default value defined on the receiver.

A field in the receiver that gets its value from the sender does not need to have the same type as that of the
corresponding sender field, provided that the receiving field's type implements `From` for the sending field's type.

For further details, see the [documentation](https://docs.rs/struct_auto_from/latest/struct_auto_from/attr.auto_from.html).

## Instalation

```toml
[dependencies]
struct_auto_from = "1"
```

## Usage

Below is a typical usage example. See the [documentation](https://docs.rs/struct_auto_from/latest/struct_auto_from/attr.auto_from.html) for additional details and examples.

```rust
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
```

## License

This library is distributed under the terms of both the MIT license and the Apache License (Version 2.0), at your
option.

See [LICENSE-APACHE](https://github.com/mateuszgrzyb/struct_auto_from/blob/master/LICENSE-APACHE)
and [LICENSE-MIT](https://github.com/mateuszgrzyb/struct_auto_from/blob/master/LICENSE-MIT),
and [COPYRIGHT](https://github.com/mateuszgrzyb/struct_auto_from/blob/master/COPYRIGHT) for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Copyrights in this project are retained by their contributors. No copyright assignment is required to contribute to this
project.

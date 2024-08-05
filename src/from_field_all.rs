use std::collections::HashMap;

use darling::{
    ast::NestedMeta,
    util::{parse_attribute_to_meta_list, path_to_string},
    Error, FromMeta, Result, ToTokens,
};
use proc_macro2::Ident;
use syn::{Expr, Field, Meta};

#[derive(Default, Debug, Clone)]
// #[darling(default, attributes(auto_from_attr))]
pub(crate) struct AutoFromAttr {
    pub(crate) default_value: Option<Expr>,
    pub(crate) from_field: Option<Ident>,
    pub(crate) from_struct: Option<Ident>,
}

impl AutoFromAttr {
    /// Returns a `Vec` corresponding to all attributes of the given field matching `Self`'s representation
    /// of a field attribute.
    ///
    /// Based on the [`darling`] crate's generated derivation of the [`darling::FromField`] trait.
    pub(crate) fn from_field_all(field: &Field, from_idents: &[Ident]) -> Result<Vec<Self>> {
        let mut errors = Error::accumulator();

        let mut my_attrs = Vec::<Self>::new();

        // control that same `from` struct not specified in multiple attributes for field
        let mut from_struct_control: HashMap<Option<Option<Ident>>, Meta> = HashMap::new();

        for attr in &field.attrs {
            let mut default_value: (Option<Meta>, Option<Option<Expr>>) = (None, None);
            let mut from_field: (Option<Meta>, Option<Option<Ident>>) = (None, None);
            let mut from_struct: (Option<Meta>, Option<Option<Ident>>) = (None, None);

            let attr_meta = &attr.meta;

            match ToString::to_string(&attr.path().clone().into_token_stream()).as_str() {
                "auto_from_attr" => match parse_attribute_to_meta_list(attr) {
                    Ok(data) => match NestedMeta::parse_meta_list(data.tokens) {
                        Ok(ref items) => {
                            for item in items {
                                match *item {
                                    NestedMeta::Meta(ref inner) => {
                                        let name = path_to_string(inner.path());

                                        match name.as_str() {
                                            "default_value" => {
                                                if default_value.0.is_none() {
                                                    default_value = (
                                                        Some(inner.clone()),
                                                        errors.handle(
                                                            FromMeta::from_meta(inner).map_err(
                                                                |e| {
                                                                    e.with_span(&inner)
                                                                        .at("default_value")
                                                                },
                                                            ),
                                                        ),
                                                    );
                                                } else {
                                                    errors.push(
                                                        Error::duplicate_field("default_value")
                                                            .with_span(&inner),
                                                    );
                                                }
                                            }
                                            "from_field" => {
                                                if from_field.0.is_none() {
                                                    from_field = (
                                                        Some(inner.clone()),
                                                        errors.handle(
                                                            FromMeta::from_meta(inner).map_err(
                                                                |e| {
                                                                    e.with_span(&inner)
                                                                        .at("from_field")
                                                                },
                                                            ),
                                                        ),
                                                    );
                                                } else {
                                                    errors.push(
                                                        Error::duplicate_field("from_field")
                                                            .with_span(&inner),
                                                    );
                                                }
                                            }
                                            "from_struct" => {
                                                if from_struct.0.is_none() {
                                                    from_struct = (
                                                        Some(inner.clone()),
                                                        errors.handle(
                                                            FromMeta::from_meta(inner).map_err(
                                                                |e| {
                                                                    e.with_span(&inner)
                                                                        .at("from_struct")
                                                                },
                                                            ),
                                                        ),
                                                    );
                                                } else {
                                                    errors.push(
                                                        Error::duplicate_field("from_struct")
                                                            .with_span(&inner),
                                                    );
                                                }
                                                if let Some(Some(ref ident)) = from_struct.1 {
                                                    if !from_idents.contains(ident) {
                                                        let ident_strs = from_idents
                                                            .iter()
                                                            .map(|ident: &Ident| ident.to_string())
                                                            .collect::<Vec<_>>();
                                                        errors.push(Error::custom(format!(
                                                            "`from_struct` value must be one of {:?}",
                                                            ident_strs
                                                        )).with_span(&inner));
                                                    }
                                                }
                                            }
                                            other => {
                                                errors.push(
                                                    Error::unknown_field_with_alts(
                                                        other,
                                                        &[
                                                            "default_value",
                                                            "from_field",
                                                            "from_struct",
                                                        ],
                                                    )
                                                    .with_span(inner),
                                                );
                                            }
                                        }
                                    }
                                    NestedMeta::Lit(ref inner) => {
                                        errors.push(
                                            Error::unsupported_format("literal").with_span(inner),
                                        );
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            errors.push(err.into());
                        }
                    },
                    Err(err) => {
                        errors.push(err);
                    }
                },
                _ => continue,
            }

            let default: Self = Default::default();
            let my_attr = Self {
                default_value: if let Some(val) = default_value.1 {
                    val
                } else {
                    default.default_value
                },
                from_field: if let Some(val) = from_field.1 {
                    val
                } else {
                    default.from_field
                },
                from_struct: if let Some(val) = from_struct.1.clone() {
                    val
                } else {
                    default.from_struct
                },
            };

            const MSG_EXACTLY_ONE: &str = concat!(
                "exactly one of `default_value` or `from_field` ",
                "must be specified for an `auto_from_attr` attribute"
            );

            if my_attr.default_value.is_some() && my_attr.from_field.is_some() {
                {
                    let meta = default_value
                        .0
                        .expect("`default_value.0` is known to be `Some(_)`");
                    errors.push(Error::custom(MSG_EXACTLY_ONE).with_span(&meta));
                }

                {
                    let meta = from_field
                        .0
                        .expect("`from_field.0` is known to be `Some(_)`");
                    errors.push(Error::custom(MSG_EXACTLY_ONE).with_span(&meta));
                }
            }

            if my_attr.default_value.is_none() && my_attr.from_field.is_none() {
                errors.push(Error::custom(MSG_EXACTLY_ONE).with_span(&attr_meta));
            }

            if let Some(meta) = from_struct_control.get_mut(&from_struct.1) {
                errors.push(
                    Error::custom(
                        "the same `from` struct may may not be used more than once for a given field"
                    )
                    .with_span(meta),
                );
            }
            from_struct_control.insert(from_struct.1, attr_meta.clone());

            my_attrs.push(my_attr);
        }

        errors.finish_with(my_attrs)
    }
}

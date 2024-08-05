//! Auto implement [`From`] trait for a receiver struct type from one or more sender struct types.
//!
//! See the [`macro@auto_from`] macro for additional details.

mod accumulator_ext;
mod from_field_all;

use crate::accumulator_ext::AccumulatorExt;
use crate::from_field_all::AutoFromAttr;

use darling::{Error, Result};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::collections::HashMap;
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, Expr, Field, ItemStruct, Meta,
    MetaList, Token,
};

/// Auto implement [`From`] trait for a receiver struct type from one or more sender struct types.
///
/// When specifying a conversion, each field in the receiver must either be defined in the sender,
/// or specify a corresponding field with a different name in the sender,
/// or have its default value defined on the receiver.
///
/// The `auto_from_attr` field attribute lets you specify the following
/// (see [Full-featured example](#full-featured-example) below):
/// - a different field name in the sender, using the `from_field` property;
/// - or a value for the the receiving field, using the `default_value` property;
/// - a `from_struct` property, in case one of the above two properties is specified, to identify the sender struct
/// type if it is not the first type in the `auto_from` macro's agument list.
///
/// A field in the receiver that gets its value from the sender does not need to have the same type as that of the
/// corresponding sender field, provided that the receiving field's type implements [`From`] for the sending field's type.
///
/// Given two structs `S` and `T`, it is perfectly fine for each of them to be the receiver of the other, e.g.,
///
/// ```ignore
/// #[auto_from(T)]
/// struct S {
///     // S's fields
/// }
///
/// #[auto_from(S)]
/// struct T {
///     // T's fields
/// }
/// ```
///
/// # Usage Examples
///
/// Note: Macro [`macro@auto_from`] doesn't require structs to implement `Debug`, `Clone`, or any other
/// traits. The `#[derive(...)]` attribute(s) in the examples below are only for purposes of the examples.
///
/// ## Basic example
///
/// ```rust
#[doc = include_str!("../examples/basic.rs")]
/// ```
///
/// ## Intermediate example
///
/// ```rust
#[doc = include_str!("../examples/intermediate.rs")]
/// ```
///
/// ## Full-featured example
///
/// ```rust
#[doc = include_str!("../examples/full_featured.rs")]
/// ```
///
#[proc_macro_attribute]
pub fn auto_from(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attrs as MacroArgs);
    let from_idents = args.idents.into_iter().collect::<Vec<_>>();

    if from_idents.is_empty() {
        return Error::custom("at least one `from` struct must be specified")
            .write_errors()
            .into();
    }

    let mut errors = Error::accumulator();

    let mut duplicate_idents = Vec::new();
    for (i, ident) in from_idents.iter().enumerate() {
        if from_idents[i + 1..].contains(ident) {
            duplicate_idents.push(ident);
        }
    }

    for ident in duplicate_idents {
        errors.push(Error::custom("duplicate identifier").with_span(ident));
    }

    let into_struct = parse_macro_input!(input as ItemStruct);

    let prelim_res = ImplData::from_parsed_input(into_struct, &from_idents);
    let final_res = errors.finnish_with_result(prelim_res);

    let ImplData {
        into_struct,
        into_ident,
        fields,
        field_attrs,
    } = match final_res {
        Ok(success) => success,
        Err(err) => return err.write_errors().into(),
    };

    let mut token_stream = TokenStream::new();

    let tokens: proc_macro2::TokenStream = quote! {
        #into_struct
    };
    let tokens = TokenStream::from(tokens);
    token_stream.extend(tokens);

    for from in from_idents {
        let empty_map = HashMap::new();
        let field_map = field_attrs.get(&from).unwrap_or(&empty_map);

        let (mapped_fields, default_fields) =
            fields.clone().into_iter().partition::<Vec<_>, _>(|ident| {
                !matches!(field_map.get(ident), Some(FieldAttr::DefaultValue(_)))
            });

        let from_fields = mapped_fields
            .iter()
            .map(|ident| match field_map.get(ident) {
                Some(FieldAttr::FromField(from_field)) => from_field.clone(),
                _ => ident.clone(),
            })
            .collect::<Vec<_>>();

        let default_values = default_fields
            .iter()
            .map(|ident| match field_map.get(ident) {
                Some(FieldAttr::DefaultValue(default_value)) => default_value,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let tokens: proc_macro2::TokenStream = quote! {
            impl From<#from> for #into_ident {
                fn from(value: #from) -> Self {
                    Self {
                        #(
                            #mapped_fields: value.#from_fields.into()
                        ),*
                        ,
                        #(
                            #default_fields: #default_values
                        ),*
                    }
                }
            }
        };
        let tokens = TokenStream::from(tokens);
        token_stream.extend(tokens);
    }

    token_stream
}

#[derive(Debug)]
struct MacroArgs {
    idents: Punctuated<Ident, Token![,]>,
}

impl Parse for MacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(MacroArgs {
            idents: input.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}

enum FieldAttr {
    FromField(Ident),
    DefaultValue(Expr),
}

/// The 1st `Ident` is the `from` type, the 2nd `Ident` is the `into` field.
type FieldAttrs = HashMap<Ident, HashMap<Ident, FieldAttr>>;

impl FieldAttr {
    fn add_to_field_attrs(
        self,
        field_attrs: &mut FieldAttrs,
        from_type: &Ident,
        into_field: &Ident,
    ) {
        let inner = field_attrs.entry(from_type.clone()).or_default();
        inner.insert(into_field.clone(), self);
    }
}

struct ImplData {
    into_struct: ItemStruct,
    into_ident: Ident,
    fields: Vec<Ident>,
    field_attrs: FieldAttrs,
}

impl ImplData {
    fn from_parsed_input(input: ItemStruct, from_idents: &[Ident]) -> Result<Self> {
        let mut into_struct = input.clone();
        let into_ident = input.ident.clone();

        let field_attrs = Self::extract_field_attributes(&mut into_struct, from_idents)?;

        let fields = input
            .fields
            .into_iter()
            .filter_map(|f| f.ident)
            .collect::<Vec<_>>();

        Ok(Self {
            into_struct,
            into_ident,
            fields,
            field_attrs,
        })
    }

    fn extract_field_attributes(
        input: &mut ItemStruct,
        from_idents: &[Ident],
    ) -> Result<FieldAttrs> {
        let mut errors = Error::accumulator();

        let mut field_attrs: FieldAttrs = HashMap::new();

        for field in input.fields.iter_mut() {
            let into_field = field
                .ident
                .clone()
                .expect("unnamed fields are not supported");

            let mut from_type = from_idents[0].clone();

            let my_attrs_res = AutoFromAttr::from_field_all(field, from_idents);
            errors.push_result(&my_attrs_res);
            let Ok(my_attrs) = my_attrs_res else {
                continue;
            };

            for my_attr in my_attrs {
                if let Some(from_struct) = my_attr.from_struct {
                    from_type = from_struct;
                }

                if let Some(from_field) = my_attr.from_field {
                    let field_attr = FieldAttr::FromField(from_field);
                    field_attr.add_to_field_attrs(&mut field_attrs, &from_type, &into_field);
                    continue;
                }

                if let Some(default_value) = my_attr.default_value {
                    let field_attr = FieldAttr::DefaultValue(default_value);
                    field_attr.add_to_field_attrs(&mut field_attrs, &from_type, &into_field);
                    continue;
                }
            }

            Self::remove_my_attrs(field);
        }

        errors.finish_with(field_attrs)
    }

    fn remove_my_attrs(field: &mut Field) {
        field.attrs.retain(|a| {
            let Meta::List(MetaList { path, .. }) = &a.meta else {
                return false;
            };

            !path.is_ident(&Ident::new("auto_from_attr", Span::call_site()))
        })
    }
}

use crate::consts::{HTTP_FAIL_RESULT, HTTP_INPUT_PARAMETER_TYPE, HTTP_REQUEST_BUCKET, NAME_SPACE};
use proc_macro::TokenStream;

use super::input_fields::InputFields;

pub fn impl_input_types(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::reflection::StructProperty::read(ast);

    let fields = InputFields::new(fields);

    let doc = super::docs::generate_http_input(&fields);

    let struct_name = name.to_string();

    let model_reader = super::model_reader::generate(struct_name.as_str(), &fields);

    let code = if fields.has_life_time() {
        format!(
            r###"impl<'s> {struct_name}<'s>{{
                pub fn get_input_params()->Vec<{NAME_SPACE}::{HTTP_INPUT_PARAMETER_TYPE}>{{
                    {doc}
                }}
                pub async fn parse_http_input(request:&'s {HTTP_REQUEST_BUCKET})->Result<{struct_name}<'s>, {HTTP_FAIL_RESULT}>{{
                    {model_reader}
                }}
        }}"###,
            struct_name = struct_name,
            doc = doc,
            model_reader = model_reader,
        )
    } else {
        format!(
            r###"impl {struct_name}{{
                pub fn get_input_params()->Vec<{NAME_SPACE}::{HTTP_INPUT_PARAMETER_TYPE}>{{
                    {doc}
                }}
                pub async fn parse_http_input(request: &{HTTP_REQUEST_BUCKET})->Result<Self, {HTTP_FAIL_RESULT}>{{
                    {model_reader}
                }}
        }}"###,
            struct_name = struct_name,
            doc = doc,
            model_reader = model_reader,
        )
    };

    code.parse().unwrap()
}

use crate::consts::*;
use proc_macro::TokenStream;

use super::input_fields::InputFields;

pub fn impl_input_types(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::reflection::StructProperty::read(ast);

    let fields = InputFields::new(fields);

    let struct_name = name.to_string();

    let code = format!(
        r###"impl {struct_name}{{
                pub fn get_input_params()->Vec<{NAME_SPACE}::{HTTP_INPUT_PARAMETER_TYPE}>{{
                    {generated_doc}
                }}
                pub async fn parse_http_input(ctx: &{HTTP_CONTEXT})->Result<Self, {HTTP_FAIL_RESULT}>{{
                    {model_reader}
                }}
        }}"###,
        struct_name = struct_name,
        generated_doc = super::docs::generate_http_input(&fields),
        model_reader = super::model_reader::generate(struct_name.as_str(), &fields),
    );

    code.parse().unwrap()
}

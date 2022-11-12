use crate::consts::*;
use proc_macro::TokenStream;

use super::input_fields::InputFields;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::reflection::StructProperty::read(ast);

    let fields = InputFields::new(fields);

    let struct_name = name.to_string();

    let mut result = String::new();
    result.push_str("impl ");
    result.push_str(struct_name.as_str());
    result.push_str("{");

    result.push_str("pub fn get_input_params()->Vec<");
    result.push_str(HTTP_INPUT_PARAMETER_TYPE_WITH_NS);
    result.push_str(">{");

    super::docs::generate_http_input(&mut result, &fields);
    result.push_str("} pub async fn parse_http_input(http_route: &my_http_server_controllers::controllers::HttpRoute, ctx: &mut ");
    result.push_str(HTTP_CONTEXT);
    result.push_str(")->Result<Self, ");
    result.push_str(HTTP_FAIL_RESULT);

    result.push_str(">{");
    super::model_reader::generate(&mut result, struct_name.as_str(), &fields);
    result.push_str("}}");

    result.parse().unwrap()
}

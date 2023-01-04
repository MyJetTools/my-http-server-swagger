use proc_macro::TokenStream;

use super::input_fields::InputFields;
use quote::quote;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let fields = types_reader::StructProperty::read(ast);

    let fields = InputFields::new(fields);

    let http_input_param = crate::consts::get_http_input_parameter_with_ns();

    let http_ctx = crate::consts::get_http_context();

    let http_fail_result = crate::consts::get_http_fail_result();

    let http_input = match super::docs::generate_http_input(&fields) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    };

    let use_doc = crate::consts::get_use_documentation();

    let parse_http_input = match super::model_reader::generate(&struct_name, &fields) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    };

    quote!{
        impl #struct_name{
            pub fn get_input_params()->Vec<#http_input_param>{
                #http_input
            }

            pub async fn parse_http_input(http_route: &my_http_server_controllers::controllers::HttpRoute, ctx: &mut #http_ctx)->Result<Self,#http_fail_result>{
                #parse_http_input
            }
        }
    }.into()
}

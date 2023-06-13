use proc_macro::TokenStream;

use quote::quote;
use types_reader::StructProperty;

use super::input_model_struct_property_ext::InputModelStructPropertyExt;

pub fn generate(ast: &syn::DeriveInput) -> (TokenStream, bool) {
    let struct_name = &ast.ident;

    let mut debug = false;

    let fields = match types_reader::StructProperty::read(ast) {
        Ok(result) => result,
        Err(err) => return (err.into_compile_error().into(), false),
    };

    for prop in &fields {
        if prop.attrs.has_attr("debug") {
            debug = true;
        }
    }

    let http_input_param = crate::consts::get_http_input_parameter_with_ns();

    let http_ctx = crate::consts::get_http_context();

    let http_fail_result = crate::consts::get_http_fail_result();

    let http_input = match super::docs::generate_http_input(&fields) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    };

    let parse_http_input = match super::model_reader::generate(&struct_name, &fields) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    };

    let http_routes = match http_routes(&fields) {
        Ok(result) => {
            if result.is_empty() {
                quote! {None}
            } else {
                quote!(Some(vec![#(#result),*]))
            }
        }
        Err(err) => err.to_compile_error(),
    };

    (quote!{
        impl #struct_name{
            pub fn get_input_params()->Vec<#http_input_param>{
                #http_input
            }

            pub async fn parse_http_input(http_route: &my_http_server_controllers::controllers::HttpRoute, ctx: &mut #http_ctx)->Result<Self,#http_fail_result>{
                #parse_http_input
            }

            pub fn get_model_routes()->Option<Vec<&'static str>>{
                #http_routes
            }
        }
    }.into(), debug)
}

fn http_routes(props: &[StructProperty]) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut result = Vec::new();

    for struct_property in props {
        if let Some(input_field) = struct_property.try_into_input_path_field()? {
            let input_field_data = input_field.get_input_data();

            let name = input_field_data.get_input_field_name()?;
            result.push(quote! {
                #name
            });
        }
    }

    Ok(result)
}

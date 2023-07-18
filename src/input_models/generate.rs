use proc_macro::TokenStream;

use quote::quote;

use super::http_input_props::HttpInputProperties;

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

    let input_fields = match HttpInputProperties::new(&fields) {
        Ok(result) => result,
        Err(err) => return (err.into_compile_error().into(), false),
    };

    let http_input_param = crate::consts::get_http_input_parameter_with_ns();

    let http_ctx = crate::consts::get_http_context();

    let http_fail_result = crate::consts::get_http_fail_result();

    let http_input = match super::docs::generate_http_input(&input_fields) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    };

    let parse_http_input = match super::model_reader::generate(&struct_name, &input_fields) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    };

    let http_routes = match http_routes(&input_fields) {
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

fn http_routes(props: &HttpInputProperties) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut result = Vec::new();

    if let Some(path_fields) = &props.path_fields {
        for input_field in path_fields {
            let name = input_field.get_input_field_name()?;
            result.push(quote! {
                #name
            });
        }
    }

    Ok(result)
}

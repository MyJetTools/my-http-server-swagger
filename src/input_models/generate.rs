use proc_macro::TokenStream;

use super::input_fields::InputFields;
use quote::quote;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let generic = &ast.generics;

    println!("generic: {:#?}", generic);

    let fields = match types_reader::StructProperty::read(ast) {
        Ok(result) => result,
        Err(err) => return err.into_compile_error().into(),
    };

    let fields = match InputFields::new(fields) {
        Ok(result) => result,
        Err(err) => return err.into_compile_error().into(),
    };

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

    let http_routes = if let Some(http_routes) = http_routes(&fields) {
        http_routes
    } else {
        quote!(None)
    };

    quote!{
        impl #generic #struct_name{
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
    }.into()
}

fn http_routes(fields: &InputFields) -> Option<proc_macro2::TokenStream> {
    let routes = fields.get_routes()?;

    let mut result = Vec::new();
    for field in routes {
        let name = field.name();
        let name = name.as_str();
        result.push(quote! {
            #name
        });
    }

    Some(quote! {
        Some(vec![
            #(#result),*
        ])
    })
}

use std::str::FromStr;

use proc_macro2::TokenStream;

use super::attributes::HttpResult;

pub fn generate(results: &[HttpResult]) -> TokenStream {
    let mut fields = Vec::new();

    let http_result_type = crate::consts::get_http_result();

    let http_data_type = crate::consts::get_http_data_type();

    for http_result in results {
        let description = &http_result.description;

        let http_code = http_result.status_code;

        let data_type = if let Some(result_type) = &http_result.result_type {
            match result_type {
                super::attributes::ResultType::Object(object_name) => generate_as_object_or_array(
                    object_name,
                    quote::quote!(into_http_data_type_object),
                ),
                super::attributes::ResultType::Array(object_name) => generate_as_object_or_array(
                    object_name,
                    quote::quote!(into_http_data_type_array),
                ),
                super::attributes::ResultType::ArrayOfSimpleType(type_name) => {
                    let http_array_element = crate::consts::get_http_array_element();
                    let http_simple_type = crate::consts::get_http_simple_type();
                    let type_name = TokenStream::from_str(type_name).unwrap();
                    quote::quote!(#http_data_type::ArrayOf(#http_array_element::SimpleType(#http_simple_type::#type_name)))
                }
                super::attributes::ResultType::SimpleType(type_name) => {
                    let http_simple_type = crate::consts::get_http_simple_type();
                    let type_name = TokenStream::from_str(type_name).unwrap();
                    quote::quote!(#http_data_type::SimpleType(#http_simple_type::#type_name))
                }
            }
        } else {
            quote::quote!(http_data_type::None)
        };

        fields.push(quote::quote! {
            #http_result_type{
                nullable: false,
                description: #description.to_string(),
                http_code: #http_code,
                data_type: #data_type,
            }
        });
    }

    quote::quote!(vec![#(#fields),*]).into()
}

fn generate_as_object_or_array(object_name: &str, into_structure: TokenStream) -> TokenStream {
    let object_name = TokenStream::from_str(object_name).unwrap();
    quote::quote!(#object_name::get_http_data_structure().#into_structure())
}

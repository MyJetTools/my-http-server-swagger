use quote::quote;

use crate::generic_utils::GenericData;

pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let stuct_name = &ast.ident;

    let result = if let Some(generic) = GenericData::new(ast) {
        let generic_token_stream = generic.generic;
        let generic_ident = generic.generic_ident;
        quote! {
            impl<'s, #generic_token_stream> TryFrom<my_http_server::InputParamValue<'s>> for #stuct_name #generic_ident {
                type Error = my_http_server::HttpFailResult;

                fn try_from(value: my_http_server::InputParamValue) -> Result<Self, Self::Error> {
                    value.from_json()
                }
            }

            impl #generic_token_stream TryFrom<my_http_server::HttpRequestBody> for #stuct_name #stuct_name #generic_ident {
                type Error = my_http_server::HttpFailResult;

                fn try_from(value: my_http_server::HttpRequestBody) -> Result<Self, Self::Error> {
                    value.get_body_as_json()
                }
            }
        }
    } else {
        quote! {
            impl<'s> TryFrom<my_http_server::InputParamValue<'s>> for #stuct_name {
                type Error = my_http_server::HttpFailResult;

                fn try_from(value: my_http_server::InputParamValue) -> Result<Self, Self::Error> {
                    value.from_json()
                }
            }

            impl TryFrom<my_http_server::HttpRequestBody> for #stuct_name {
                type Error = my_http_server::HttpFailResult;

                fn try_from(value: my_http_server::HttpRequestBody) -> Result<Self, Self::Error> {
                    value.get_body_as_json()
                }
            }
        }
    };

    result.into()
}

use std::str::FromStr;

use quote::{quote, ToTokens};
use types_reader::StructProperty;

use super::struct_prop_ext::SturctPropertyExt;

pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let stuct_name = &ast.ident;
    let generic = &ast.generics;

    let (generic, generic_ident) = if generic.params.is_empty() {
        (None, None)
    } else {
        let generic_ident = generic.params.to_token_stream().to_string();
        let generic_ident_pos = generic_ident.find(':').unwrap();

        let gen = &generic_ident.as_bytes()[..generic_ident_pos];
        let gen = std::str::from_utf8(gen).unwrap();
        println!("generic_ident: {}", gen);

        let generic_ident = proc_macro2::TokenStream::from_str(gen).unwrap();

        (Some(quote!(#generic)), Some(quote!(<#generic_ident>)))
    };

    let result = if let Some(generic) = generic {
        let generic_ident = generic_ident.unwrap();
        quote! {
            impl<'s, #generic> TryFrom<my_http_server::InputParamValue<'s>> for #stuct_name #generic_ident {
                type Error = my_http_server::HttpFailResult;

                fn try_from(value: my_http_server::InputParamValue) -> Result<Self, Self::Error> {
                    value.from_json()
                }
            }

            impl #generic TryFrom<my_http_server::HttpRequestBody> for #stuct_name #stuct_name #generic_ident {
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

pub fn generate_http_object_structure(
    fields: Vec<StructProperty>,
) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::new();

    for field in fields {
        let line = crate::types::compile_http_field(field.get_name().as_str(), &field.ty, None);

        result.push(line);
    }

    result
}

fn render_obj_fields(fields: &[StructProperty]) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::with_capacity(fields.len());
    for field in fields {
        let line = crate::types::compile_http_field(field.get_name().as_str(), &field.ty, None);

        result.push(quote!(__hos.fields.push(#line);));
    }

    result
}

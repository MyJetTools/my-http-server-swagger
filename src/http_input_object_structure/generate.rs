use quote::quote;
use types_reader::StructProperty;

use super::struct_prop_ext::SturctPropertyExt;

pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let stuct_name = &ast.ident;

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
    .into()
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

use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;
use types_reader::{attribute_params::ParamValue, PropertyType};

use crate::as_token_stream::AsTokenStream;

pub fn compile_http_field(
    name: &str,
    pt: &PropertyType,
    required: bool,
    default: Option<ParamValue>,
) -> TokenStream {
    let data_type = compile_data_type(pt);

    let default = default.as_token_stream();
    let http_field_type = crate::consts::get_http_field_type();
    quote! {
        #http_field_type::new(#name, #data_type, #required, #default)
    }
}

pub fn compile_http_field_with_object(
    name: &str,
    body_type: &str,
    required: bool,
    default: Option<ParamValue>,
) -> TokenStream {
    let http_field_type = crate::consts::get_http_field_type();

    let default = default.as_token_stream();

    let body_type = if body_type == "file" {
        quote!(data_types::HttpDataType::SimpleType(
            data_types::HttpSimpleType::Binary
        ))
    } else {
        let body_type = proc_macro2::TokenStream::from_str(body_type).unwrap();
        quote!(#body_type::get_http_data_structure().into_http_data_type_object())
    };

    quote! {
        #http_field_type::new(#name, #body_type, #required, #default)
    }
}

fn compile_data_type(pt: &PropertyType) -> TokenStream {
    if let PropertyType::OptionOf(generic_type) = pt {
        let type_token = generic_type.get_token_stream();

        return quote!(#type_token::get_data_type());
    }

    let type_token = pt.get_token_stream();

    return quote!(#type_token::get_data_type());
}

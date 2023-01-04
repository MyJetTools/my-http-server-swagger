use std::str::FromStr;

use macros_utils::ParamValue;
use proc_macro2::TokenStream;
use quote::quote;
use types_reader::PropertyType;

use crate::as_token_stream::AsTokenStream;
use crate::proprety_type_ext::PropertyTypeExt;

pub fn compile_http_field(
    name: &str,
    pt: &PropertyType,
    required: bool,
    default: Option<ParamValue>,
) -> TokenStream {
    let data_type = compile_data_type(name, pt, TypeIsWrappedTo::None);

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

enum TypeIsWrappedTo {
    None,
    Option,
    Vec,
}

fn compile_data_type(prop_name: &str, pt: &PropertyType) -> TokenStream {
    if let PropertyType::OptionOf(generic_type) = pt {
        let type_token = generic_type.get_token_stream();
        return quote!(#type_token::get_data_type());
        //    return compile_data_type(prop_name, generic_type.as_ref(), TypeIsWrappedTo::Option);
    }

    /*
       if let PropertyType::VecOf(generic_type) = pt {
           return compile_data_type(prop_name, generic_type.as_ref(), TypeIsWrappedTo::Vec);
       }
    */
    let type_token = pt.get_token_stream();

    return quote!(#type_token::get_data_type());

    /*
    if let Some(simple_type) =
        pt.get_swagger_simple_type(prop_name.to_lowercase().contains("password"))
    {
        match type_is_wrapped_to {
            TypeIsWrappedTo::None => {
                let http_data_type = crate::consts::get_http_data_type();
                return quote! {
                    #http_data_type::SimpleType(#simple_type)
                };
            }

            TypeIsWrappedTo::Option => {
                let http_data_type = crate::consts::get_http_data_type();
                return quote! {
                    #http_data_type::SimpleType(#simple_type)
                };
            }
            TypeIsWrappedTo::Vec => {
                let http_data_type = crate::consts::get_http_data_type();
                let http_array_element = crate::consts::get_http_array_element();
                return quote! {
                    #http_data_type::ArrayOf(#http_array_element::SimpleType(#simple_type))
                };
            }
        };
    }

    match type_is_wrapped_to {
        TypeIsWrappedTo::None => {
            let tp = pt.get_token_stream();
            return quote!(#tp::get_http_data_structure().into_http_data_type_object());
        }
        TypeIsWrappedTo::Option => {
            let tp = pt.get_token_stream();
            return quote!(#tp::get_http_data_structure().into_http_data_type_object());
        }
        TypeIsWrappedTo::Vec => {
            let tp = pt.get_token_stream();
            return quote!(#tp::get_http_data_structure().into_http_data_type_object());
        }
    }
     */
}

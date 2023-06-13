use proc_macro2::TokenStream;
use quote::quote;
use types_reader::PropertyType;

use crate::{as_token_stream::AsTokenStream, property_type_ext::PropertyTypeExt};

pub fn compile_http_field(
    name: &str,
    pt: &PropertyType,
    default: Option<&str>,
) -> Result<TokenStream, syn::Error> {
    let data_type = compile_data_type(pt);
    let required = pt.required();

    let default = default.as_token_stream()?;
    let http_field_type = crate::consts::get_http_field_type();

    let result = quote! {
        #http_field_type::new(#name, #data_type, #required, #default)
    };

    Ok(result)
}

fn compile_data_type(pt: &PropertyType) -> TokenStream {
    if let PropertyType::OptionOf(generic_type) = pt {
        let type_token = generic_type.get_token_stream_with_generics();

        return quote!(#type_token::get_data_type(None));
    }

    let type_token = pt.get_token_stream_with_generics();

    return quote!(#type_token::get_data_type(None));
}

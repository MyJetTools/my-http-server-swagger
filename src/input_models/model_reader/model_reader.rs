use proc_macro2::{Ident, TokenStream};

use crate::{
    input_models::input_fields::{InputFieldSource, InputFields},
    proprety_type_ext::PropertyTypeExt,
};
use quote::quote;

pub fn generate(name: &Ident, input_fields: &InputFields) -> Result<TokenStream, syn::Error> {
    let fileds = input_fields.get_body_and_not_body_fields();

    let reading_no_body = if let Some(not_body_fields) = fileds.not_body_fields {
        Some(super::generate_read_not_body(&not_body_fields))
    } else {
        None
    };

    let has_body_data_to_read = input_fields.has_body_data_to_read()?;

    let read_body = if let Some(body_data_to_read) = has_body_data_to_read {
        let body_fields = fileds.body_fields.as_ref().unwrap();
        if body_data_to_read.form_data_field > 0 {
            Some(super::generate_read_body(body_fields))
        } else if body_data_to_read.body_field > 0 {
            Some(super::generate_read_body(body_fields))
        } else {
            None
        }
    } else {
        None
    };

    let mut fileds_to_return = Vec::new();

    for input_field in &input_fields.fields {
        match &input_field.src {
            InputFieldSource::Query => {
                fileds_to_return.push(input_field.get_struct_fiel_name_as_token_stream());
            }
            InputFieldSource::Path => {
                fileds_to_return.push(input_field.get_struct_fiel_name_as_token_stream());
            }
            InputFieldSource::Header => {
                fileds_to_return.push(input_field.get_struct_fiel_name_as_token_stream());
            }
            InputFieldSource::Body => {
                if input_field.property.ty.is_file_content() {
                    let struct_field_name = input_field.property.get_field_name_ident();
                    let item =
                        quote!(#struct_field_name : ctx.request.get_body().await?.get_body()?);
                    fileds_to_return.push(item);
                } else if input_field.property.ty.is_vec_of_u8() {
                    let struct_field_name = input_field.property.get_field_name_ident();
                    let item =
                        quote!(#struct_field_name : ctx.request.receive_body().await?.get_body());
                    fileds_to_return.push(item);
                } else if input_field.property.ty.is_struct() {
                    let struct_field_name = input_field.property.get_field_name_ident();
                    let item = quote!(#struct_field_name : ctx.request.get_body().await?.get_body_as_json()?);
                    fileds_to_return.push(item);
                } else {
                    fileds_to_return.push(input_field.get_struct_fiel_name_as_token_stream());
                }
            }
            InputFieldSource::FormData => {
                fileds_to_return.push(input_field.get_struct_fiel_name_as_token_stream());
            }
        }
    }

    let result = quote! {
        #reading_no_body
        #read_body
        Ok(#name{
            #(#fileds_to_return),*
        })
    };
    Ok(result)
}

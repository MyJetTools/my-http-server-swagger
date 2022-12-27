use proc_macro2::{Ident, TokenStream};

use crate::input_models::input_fields::{BodyDataToReader, InputFieldSource, InputFields};
use quote::quote;

pub fn generate(name: &Ident, input_fields: &InputFields) -> Result<TokenStream, syn::Error> {
    let fileds = input_fields.get_body_and_not_body_fields();

    let reading_no_body = if let Some(not_body_fields) = fileds.not_body_fields {
        Some(super::generate_read_not_body(&not_body_fields))
    } else {
        None
    };

    let has_body_data_to_read = input_fields.has_body_data_to_read();

    let read_body = if let Some(body_fields) = fileds.body_fields {
        if let Some(body_data_reader_type) = &has_body_data_to_read {
            match body_data_reader_type {
                BodyDataToReader::FormData => Some(super::generate_read_body(&body_fields)),

                BodyDataToReader::BodyModel => Some(super::generate_read_body(&body_fields)),
                _ => None,
            }
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
                if let Some(has_body_data_to_read) = &has_body_data_to_read {
                    match has_body_data_to_read {
                        BodyDataToReader::FormData => {
                            fileds_to_return
                                .push(input_field.get_struct_fiel_name_as_token_stream());
                        }
                        BodyDataToReader::BodyFile => {
                            let struct_field_name = input_field.property.get_field_name_ident();
                            let item = quote!(#struct_field_name : ctx.request.get_body().await?.get_body_as_json()?);
                            fileds_to_return.push(item);
                        }
                        BodyDataToReader::RawBodyToVec => {
                            let struct_field_name = input_field.property.get_field_name_ident();
                            let item = quote!(#struct_field_name : ctx.request.receive_body().await?.get_body());
                            fileds_to_return.push(item);
                        }
                        BodyDataToReader::DeserializeBody => {
                            let struct_field_name = input_field.property.get_field_name_ident();
                            let item = quote!(#struct_field_name : ctx.request.get_body().await?.get_body_as_json()?);
                            fileds_to_return.push(item);
                        }
                        BodyDataToReader::BodyModel => {
                            fileds_to_return
                                .push(input_field.get_struct_fiel_name_as_token_stream());
                        }
                    }
                }
            }
            InputFieldSource::FormData => {
                fileds_to_return.push(input_field.get_struct_fiel_name_as_token_stream());
            }
            InputFieldSource::BodyFile => {
                let struct_field_name = input_field.property.get_field_name_ident();
                let item =
                    quote!(#struct_field_name : ctx.request.receive_body().await?.get_body());
                fileds_to_return.push(item);
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

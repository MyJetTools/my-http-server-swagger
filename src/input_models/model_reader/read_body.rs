use std::str::FromStr;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use types_reader::PropertyType;

use crate::{
    as_token_stream::AsTokenStream,
    input_models::input_fields::{InputField, InputFieldSource},
    proprety_type_ext::PropertyTypeExt,
};

pub fn get_body_data_src() -> TokenStream {
    quote!(__reader)
}
pub fn generate_read_body(input_fields: &Vec<&InputField>) -> Result<TokenStream, syn::Error> {
    let data_src = get_body_data_src();

    let mut validation = Vec::with_capacity(input_fields.len());

    let mut reading_feilds = Vec::with_capacity(input_fields.len());

    for input_field in input_fields {
        let struct_field_name = input_field.property.get_field_name_ident();

        match &input_field.property.ty {
            PropertyType::OptionOf(sub_type) => {
                let input_field_name = input_field.name();
                let input_field_name = input_field_name.as_str();

                let sub_type = sub_type.get_token_stream();

                let line = quote::quote! {
                    let #struct_field_name = if let Some(value) = #data_src.get_optional(#input_field_name){
                        let value: #sub_type = value.try_into()?;
                        Some(value)
                    }else{
                        None
                    };
                };

                reading_feilds.push(line);
            }
            PropertyType::VecOf(_) => {}
            PropertyType::Struct(..) => {
                if input_field.property.ty.is_file_content() {
                    reading_feilds.push(generate_reading_required(
                        input_field,
                        &data_src,
                        &struct_field_name,
                    )?);
                } else {
                    let input_field_name = input_field.name();
                    let input_field_name = input_field_name.as_str();

                    let line = quote!(let #struct_field_name = #data_src.get_required(#input_field_name)?.try_into()?;);
                    reading_feilds.push(line);
                }
            }
            _ => {
                reading_feilds.push(generate_reading_required(
                    input_field,
                    &data_src,
                    &struct_field_name,
                )?);
            }
        }

        if let Some(validator) = input_field.validator() {
            let validation_fn_name =
                proc_macro2::TokenStream::from_str(validator.as_str()).unwrap();
            validation.push(quote!(#validation_fn_name(ctx, &#struct_field_name)?;));
        }
    }

    let init_fields = input_fields.as_token_stream();

    let result = quote! {
        let #init_fields ={
            let __body = ctx.request.get_body().await?;
            let __reader = __body.get_body_data_reader()?;
            #(#reading_feilds)*
            #init_fields
        };

        #(#validation)*
    };

    Ok(result)
}

fn generate_reading_required(
    input_field: &InputField,
    data_src: &TokenStream,
    struct_field: &Ident,
) -> Result<TokenStream, syn::Error> {
    let result = match input_field.src {
        InputFieldSource::Query => {
            panic!("Bug. Query is not supported for read body model");
        }
        InputFieldSource::Path => {
            panic!("Bug. Path is not supported for read body model");
        }
        InputFieldSource::Header => {
            panic!("Bug. Path is not supported for read body model");
        }
        InputFieldSource::Body => {
            let input_field_name = input_field.name();
            let input_field_name = input_field_name.as_str();

            quote!(let #struct_field = #data_src.get_required(#input_field_name)?.try_into()?;)
        }
        InputFieldSource::FormData => {
            let input_field_name = input_field.name();
            let input_field_name = input_field_name.as_str();

            quote!(let #struct_field = #data_src.get_required(#input_field_name)?.try_into()?;)
        }
    };

    Ok(result)
}

use std::str::FromStr;

use proc_macro2::TokenStream;
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
pub fn generate_read_body(input_fields: &Vec<&InputField>) -> TokenStream {
    let data_src = get_body_data_src();

    let mut validation = Vec::with_capacity(input_fields.len());

    let mut reading_feilds = Vec::with_capacity(input_fields.len());

    for input_field in input_fields {
        let struct_field_name = input_field.property.get_field_name_ident();

        println!(
            "input_field: {}. ty: {}",
            struct_field_name.to_string(),
            input_field.property.ty.get_token_stream()
        );

        match &input_field.property.ty {
            PropertyType::OptionOf(sub_type) => {
                let input_field_name = input_field.name();
                let input_field_name = input_field_name.get_value_as_str();

                let sub_type = sub_type.get_token_stream();

                let line = quote::quote! {
                    let #struct_field_name = if let Some(value) = #data_src.get_optional(#input_field_name){
                        let value: #sub_type = value.try_into()?;
                        Some(value)
                    }else{
                        None
                    }
                };

                reading_feilds.push(line);
            }
            PropertyType::VecOf(_) => {}
            PropertyType::Struct(_, ty) => {
                if input_field.property.ty.is_file_content() {
                    let line = generate_reading_required(input_field, &data_src);
                    reading_feilds.push(line);
                } else {
                    let input_field_name = input_field.name();
                    let input_field_name = input_field_name.get_value_as_str();

                    let line = quote::quote! {
                        let #struct_field_name = #data_src.get_required(#input_field_name)?;
                        let #struct_field_name: #ty = #struct_field_name.try_into()?;
                    };

                    reading_feilds.push(line);
                }
            }
            _ => {
                reading_feilds.push(generate_reading_required(input_field, &data_src));
            }
        }

        if let Some(validator) = input_field.validator() {
            let validation_fn_name =
                proc_macro2::TokenStream::from_str(validator.get_value_as_str()).unwrap();
            validation.push(quote!(#validation_fn_name(ctx, &#struct_field_name)?;));
        }
    }

    let init_fields = input_fields.as_token_stream();

    quote! {
        let #init_fields ={
            let __body = ctx.request.get_body().await?;
            #(#reading_feilds)*
            #init_fields
        };

        #(#validation)*
    }
}

fn generate_reading_required(input_field: &InputField, data_src: &TokenStream) -> TokenStream {
    match input_field.src {
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
            let input_field_name = input_field_name.get_value_as_str();

            quote!(#data_src.get_required(#input_field_name)?.try_into()?;)
        }
        InputFieldSource::FormData => {
            let input_field_name = input_field.name();
            let input_field_name = input_field_name.get_value_as_str();

            quote!(#data_src.get_required(#input_field_name)?.try_into()?;)
        }
        InputFieldSource::BodyFile => {
            panic!("Bug. Should not read BodyFile at read body model");
        }
    }
}

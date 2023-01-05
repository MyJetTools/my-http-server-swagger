use proc_macro2::{Ident, TokenStream};
use types_reader::PropertyType;

use crate::input_models::input_fields::{InputField, InputFieldSource, InputFields};
use quote::quote;

pub fn generate(name: &Ident, input_fields: &InputFields) -> Result<TokenStream, syn::Error> {
    let fileds = input_fields.get_body_and_not_body_fields();

    let reading_no_body = if let Some(not_body_fields) = fileds.not_body_fields {
        Some(super::generate_read_not_body(&not_body_fields))
    } else {
        None
    };

    let has_body_data_to_read = input_fields.has_body_data_to_read()?;

    let read_body = if let Some(body_data_to_read) = &has_body_data_to_read {
        let body_fields = fileds.body_fields.as_ref().unwrap();
        if body_data_to_read.http_form > 1 || body_data_to_read.http_body > 1 {
            Some(super::generate_read_body(body_fields)?)
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
                let body_data_to_read = has_body_data_to_read.as_ref().unwrap();

                if body_data_to_read.http_body > 1 {
                    fileds_to_return.push(input_field.get_struct_fiel_name_as_token_stream());
                } else {
                    fileds_to_return.push(read_body_single_field(input_field));
                }
            }
            InputFieldSource::BodyRaw => {
                let struct_field_name = input_field.get_struct_fiel_name_as_token_stream();
                let read_value = read_from_body_raw(input_field)?;
                fileds_to_return.push(quote!(#struct_field_name: #read_value));
            }
            InputFieldSource::FormData => {
                let body_data_to_read = has_body_data_to_read.as_ref().unwrap();

                if body_data_to_read.http_form > 1 {
                    fileds_to_return.push(input_field.get_struct_fiel_name_as_token_stream());
                } else {
                    let struct_field_name = input_field.get_struct_fiel_name_as_token_stream();
                    let read_value = read_from_form_data_as_single_field(input_field)?;
                    fileds_to_return.push(quote!(#struct_field_name: #read_value));
                }
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

fn read_from_body_raw(input_field: &InputField) -> Result<TokenStream, syn::Error> {
    if input_field.property.ty.is_option() {
        let field_name = input_field.name();
        let field_name = field_name.as_str();

        let result = quote!({
            let body = ctx.request.receive_body().await?;
            let body_reader = body.get_body_data_reader()?;

            if let Some(value) = body_reader.get_optional(#field_name){
                Some(value.try_into()?)
            }else{
                None
            }
        });
        return Ok(result);
    }
    let result = quote!(ctx.request.receive_body().await?.into());
    return Ok(result);
}

fn read_from_form_data_as_single_field(
    input_field: &InputField,
) -> Result<TokenStream, syn::Error> {
    let field_name = input_field.name();
    let field_name = field_name.as_str();

    if input_field.property.ty.is_option() {
        let result = quote!({
            let body = ctx.request.receive_body().await?;
            let body_reader = body.get_body_data_reader()?;

            if let Some(value) = body_reader.get_optional(#field_name){
                Some(value.try_into()?)
            }else{
                None
            }
        });
        return Ok(result);
    }
    let result = quote!({
        let body = ctx.request.receive_body().await?;
        let body_reader = body.get_body_data_reader()?;
        body_reader.get_required(#field_name)?.try_into()?
    });
    return Ok(result);
}

fn read_body_single_field(input_field: &InputField) -> proc_macro2::TokenStream {
    let struct_field_name = input_field.get_struct_fiel_name_as_token_stream();
    let field_name = input_field.name();
    let field_name = field_name.as_str();

    if let PropertyType::OptionOf(_) = &input_field.property.ty {
        return quote!(
            #struct_field_name: {
                let data_reader = ctx.request.get_body().await?.get_body_data_reader()?;
                if let Some(value) =data_reader.get_optional(#field_name){
                    Some(value.try_into()?)
                }
                else{
                    None
                }
            }

        );
    }

    quote!(#struct_field_name: {
        let data_reader = ctx.request.get_body().await?.get_body_data_reader()?;
        let value = data_reader.get_required(#field_name)?;
        value.try_into()?
    })
}

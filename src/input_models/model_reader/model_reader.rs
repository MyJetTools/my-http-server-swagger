use proc_macro2::{Ident, TokenStream};
use types_reader::{PropertyType, StructProperty};

use quote::quote;

use crate::input_models::{
    body_nor_body_fields::BodyNotBodyFields,
    input_model_struct_property_ext::InputModelStructPropertyExt, InputField, InputFieldData,
};

pub fn generate(name: &Ident, properties: &[StructProperty]) -> Result<TokenStream, syn::Error> {
    let fields = BodyNotBodyFields::new(properties)?;

    let reading_no_body = if let Some(not_body_fields) = &fields.not_body_fields {
        Some(super::generate_read_not_body(&not_body_fields)?)
    } else {
        None
    };

    let has_body_data_to_read = fields.has_body_data_to_read()?;
    let read_body = if let Some(body_data_to_read) = &has_body_data_to_read {
        let body_fields = fields.body_fields.as_ref().unwrap();
        if body_data_to_read.http_form > 1 || body_data_to_read.http_body > 1 {
            Some(super::generate_read_body(body_fields)?)
        } else {
            None
        }
    } else {
        None
    };

    let mut fields_to_return = Vec::new();

    for struct_property in properties {
        let input_field = struct_property.get_input_field()?;

        match &input_field {
            InputField::Query(_) => {
                fields_to_return.push(struct_property.get_struct_field_name_as_token_stream());
            }
            InputField::Path(field_data) => {
                let input_field_name = field_data.get_input_field_name();
                let input_field_name = input_field_name.as_str();
                //          quote!(let #struct_field_name = http_route.get_value(&ctx.request.http_path, #input_field_name)?.try_into()?;)

                let struct_field_name = struct_property.get_struct_field_name_as_token_stream();
                fields_to_return.push(quote!(#struct_field_name: http_route.get_value(&ctx.request.http_path, #input_field_name)?.try_into()?));
            }
            InputField::Header(_) => {
                fields_to_return.push(struct_property.get_struct_field_name_as_token_stream());
            }
            InputField::Body(field_data) => {
                let body_data_to_read = has_body_data_to_read.as_ref().unwrap();

                if body_data_to_read.http_body > 1 {
                    fields_to_return.push(struct_property.get_struct_field_name_as_token_stream());
                } else {
                    fields_to_return.push(read_body_single_field(struct_property, field_data));
                }
            }
            InputField::BodyRaw(field_data) => {
                let struct_field_name = struct_property.get_struct_field_name_as_token_stream();
                let read_value = read_from_body_raw(struct_property, field_data)?;
                fields_to_return.push(quote!(#struct_field_name: #read_value));
            }
            InputField::FormData(field_data) => {
                let body_data_to_read = has_body_data_to_read.as_ref().unwrap();

                if body_data_to_read.http_form > 1 {
                    fields_to_return.push(struct_property.get_struct_field_name_as_token_stream());
                } else {
                    let struct_field_name = struct_property.get_struct_field_name_as_token_stream();
                    let read_value =
                        read_from_form_data_as_single_field(struct_property, field_data)?;
                    fields_to_return.push(quote!(#struct_field_name: #read_value));
                }
            }
        }
    }

    let result = quote! {
        #reading_no_body
        #read_body
        Ok(#name{
            #(#fields_to_return),*
        })
    };

    Ok(result)
}

fn read_from_body_raw(
    struct_property: &StructProperty,
    input_field_data: &InputFieldData,
) -> Result<TokenStream, syn::Error> {
    if struct_property.ty.is_option() {
        let field_name = input_field_data.get_input_field_name();
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
    let result = quote!(ctx.request.receive_body().await?.try_into()?);
    return Ok(result);
}

fn read_from_form_data_as_single_field(
    struct_property: &StructProperty,
    input_field_data: &InputFieldData,
) -> Result<TokenStream, syn::Error> {
    let field_name = input_field_data.get_input_field_name();
    let field_name = field_name.as_str();

    if struct_property.ty.is_option() {
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

fn read_body_single_field(
    struct_property: &StructProperty,
    input_field_data: &InputFieldData,
) -> proc_macro2::TokenStream {
    let struct_field_name = struct_property.get_struct_field_name_as_token_stream();
    let field_name = input_field_data.get_input_field_name();
    let field_name = field_name.as_str();

    if let PropertyType::OptionOf(_) = &struct_property.ty {
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

use proc_macro2::{Ident, TokenStream};
use types_reader::PropertyType;

use quote::quote;

use crate::input_models::{
    http_input_props::HttpInputProperties,
    input_model_struct_property_ext::InputModelStructPropertyExt, InputField,
};

pub fn generate(name: &Ident, properties: &HttpInputProperties) -> Result<TokenStream, syn::Error> {
    let mut fields_to_return = Vec::new();
    if let Some(path_fields) = &properties.path_fields {
        for input_field in path_fields {
            let input_field_name = input_field.get_input_field_name()?;
            let struct_field_name = input_field.property.get_struct_field_name_as_token_stream();
            fields_to_return.push(quote!(#struct_field_name: http_route.get_value(&ctx.request.http_path, #input_field_name)?.try_into()?));
        }
    };

    let reading_headers = if let Some(header_fields) = &properties.header_fields {
        let mut result = Vec::with_capacity(header_fields.len());
        for input_field in header_fields {
            let struct_field_name = input_field.property.get_field_name_ident();
            result.push(read_header_field(input_field)?);
            fields_to_return.push(quote!(#struct_field_name));
        }

        quote!(#(#result)*)
    } else {
        quote!()
    };

    let reading_query_string = if let Some(query_string_fields) = &properties.query_string_fields {
        for input_field in query_string_fields {
            let struct_field_name = input_field.property.get_field_name_ident();
            fields_to_return.push(quote!(#struct_field_name));
        }
        super::generate_read_not_body(query_string_fields.as_slice(), || quote!(__query_string))?
    } else {
        quote!()
    };

    if let Some(body_raw_field) = &properties.body_raw_field {
        let struct_field_name = body_raw_field
            .property
            .get_struct_field_name_as_token_stream();
        let read_value = read_from_body_raw(&body_raw_field)?;
        fields_to_return.push(quote!(#struct_field_name: #read_value));
    };

    let read_body = if let Some(body_fields) = &properties.body_fields {
        if body_fields.len() > 1 {
            for input_field in body_fields {
                let struct_field_name = input_field.property.get_field_name_ident();
                fields_to_return.push(quote!(#struct_field_name));
            }

            super::read_body::generate_read_body(body_fields)?
        } else {
            fields_to_return.push(read_body_single_field(body_fields.get(0).unwrap())?);
            quote!()
        }
    } else {
        quote!()
    };

    let read_form_data = if let Some(form_data_fields) = &properties.form_data_fields {
        if form_data_fields.len() > 1 {
            for input_field in form_data_fields {
                let struct_field_name = input_field.property.get_field_name_ident();
                fields_to_return.push(quote!(#struct_field_name));
            }

            super::read_body::generate_read_body(form_data_fields)?
        } else {
            let input_field = form_data_fields.get(0).unwrap();
            let struct_field_name = input_field.property.get_struct_field_name_as_token_stream();
            let read_value = read_from_form_data_as_single_field(input_field)?;
            fields_to_return.push(quote!(#struct_field_name: #read_value));
            quote!()
        }
    } else {
        quote!()
    };

    /*
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


       for struct_property in properties {
           let input_field = struct_property.get_input_field()?;

           match &input_field {
               InputField::Query(_) => {
                   fields_to_return.push(struct_property.get_struct_field_name_as_token_stream());
               }
               InputField::Path(field_data) => {
                   let input_field_name = field_data.get_input_field_name()?;

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
                       fields_to_return.push(read_body_single_field(struct_property, field_data)?);
                   }
               }
               InputField::BodyRaw(field_data) => {}
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
    */

    let result = quote! {
        #reading_headers
        #reading_query_string
        #read_body
        #read_form_data
        Ok(#name{
            #(#fields_to_return),*
        })
    };

    Ok(result)
}

fn read_from_body_raw(input_field: &InputField) -> Result<TokenStream, syn::Error> {
    if input_field.property.ty.is_option() {
        let field_name = input_field.get_input_field_name()?;

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
    input_field: &InputField,
) -> Result<TokenStream, syn::Error> {
    let field_name = input_field.get_input_field_name()?;

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

fn read_body_single_field(
    input_field: &InputField,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let struct_field_name = input_field.property.get_struct_field_name_as_token_stream();
    let field_name = input_field.get_input_field_name()?;

    if let PropertyType::OptionOf(_) = &input_field.property.ty {
        let result = quote!(
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

        return Ok(result);
    }

    let result = quote!(#struct_field_name: {
        let data_reader = ctx.request.get_body().await?.get_body_data_reader()?;
        let value = data_reader.get_required(#field_name)?;
        value.try_into()?
    });

    Ok(result)
}

fn read_header_field(input_field: &InputField) -> Result<proc_macro2::TokenStream, syn::Error> {
    let input_field_name = input_field.get_input_field_name()?;
    let struct_field_name = input_field.property.get_struct_field_name_as_token_stream();

    if input_field.property.ty.is_option() {
        let default_value = input_field.get_default_value_opt_case()?;

        let result = quote! {
            let #struct_field_name = if let Some(value) = ctx.request.get_optional_header(#input_field_name) {
                Some(value.try_into()?)
            } else {
                #default_value
            };
        };

        return Ok(result);
    }

    if !input_field.has_default_value() {
        let result = quote!(let #struct_field_name = ctx.request.get_required_header(#input_field_name)?.try_into()?;);
        return Ok(result);
    }

    let default_value = input_field.get_default_value_non_opt_case()?;

    let result = quote! {
        let #struct_field_name = if let Some(value) = ctx.request.get_optional_header(#input_field_name) {
            Some(value.try_into()?)
        } else {
            #default_value
        };
    };

    return Ok(result);
}

use std::str::FromStr;

use crate::as_token_stream::AsTokenStream;
use proc_macro2::TokenStream;
use types_reader::PropertyType;

use crate::input_models::input_fields::{InputField, InputFieldSource};

use quote::quote;

pub fn get_query_string_data_src() -> TokenStream {
    quote!(__query_string)
}

pub fn generate_read_not_body(input_fields: &Vec<&InputField>) -> Result<TokenStream, syn::Error> {
    let mut validation = Vec::with_capacity(input_fields.len());
    let data_src = get_query_string_data_src();

    let mut reading_feilds: Vec<TokenStream> = Vec::with_capacity(input_fields.len());

    for input_field in input_fields {
        let struct_field_name = input_field.property.get_field_name_ident();

        match &input_field.property.ty {
            PropertyType::OptionOf(_) => {
                let input_field_name = input_field.name();
                let input_field_name = input_field_name.as_str();

                let item = quote! {
                    let #struct_field_name = if let Some(value) = #data_src.get_optional(#input_field_name){
                        let value = my_http_server::InputParamValue::from(value);
                        Some(value.try_into()?)
                    }else{
                        None
                    };
                }.into();

                reading_feilds.push(item);
            }
            PropertyType::VecOf(sub_type) => {
                if sub_type.is_string() {
                    let input_field_name = input_field.name();
                    let input_field_name = input_field_name.as_str();

                    let item = quote! {
                      let #struct_field_name = #data_src.get_vec_of_string(#input_field_name)?;
                    }
                    .into();

                    reading_feilds.push(item);
                } else {
                    let input_field_name = input_field.name();
                    let input_field_name = input_field_name.as_str();

                    let item = quote! {
                       let #struct_field_name = #data_src.get_vec(#input_field_name)?;
                    }
                    .into();

                    reading_feilds.push(item);
                }
            }
            PropertyType::Struct(..) => {
                let input_field_name = input_field.name();
                let input_field_name = input_field_name.as_str();

                let prop_type = input_field.property.get_syn_type();

                let default_value = if let Some(default_value) = input_field.get_default_value() {
                    let value = default_value.as_str();
                    if value == "" {
                        Some(quote!(#prop_type::create_default()?))
                    } else {
                        Some(quote!(<#prop_type as std::str::FromStr>::from_str(#value)?))
                    }
                } else {
                    None
                };

                match default_value {
                    Some(default_value) => {
                        let item = quote! {
                           let #struct_field_name = match #data_src.get_optional(#input_field_name){
                            Some(value) =>{
                                let value = my_http_server::InputParamValue::from(value);
                                value.try_into()?
                            },
                            None => {
                                #default_value
                            }
                           };

                        }
                        .into();

                        reading_feilds.push(item);
                    }
                    None => {
                        reading_feilds.push(generate_reading_required(input_field)?);
                    }
                }
            }
            _ => {
                reading_feilds.push(generate_reading_required(input_field)?);
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
        let #init_fields = {
            let #data_src = ctx.request.get_query_string()?;
            #(#reading_feilds)*
            #init_fields
        };
        #(#validation)*
    };

    Ok(result)
}

fn generate_reading_required(input_field: &InputField) -> Result<TokenStream, syn::Error> {
    let struct_field_name = input_field.property.get_field_name_ident();
    match input_field.src {
        InputFieldSource::Query => {
            let data_src = get_query_string_data_src();
            let input_field_name = input_field.name();
            let input_field_name = input_field_name.as_str();
            if let Some(default) = input_field.get_default_value() {
                let else_data = proc_macro2::TokenStream::from_str(default.as_str());

                if let Err(err) = else_data {
                    return Err(syn::Error::new_spanned(
                        input_field.property.field,
                        format!("Invalid default value: {}", err),
                    ));
                }

                let else_data = else_data.unwrap();

                let result = quote! {
                    let #struct_field_name = if let Some(value) = #data_src.get_optional(#input_field_name){
                        my_http_server::InputParamValue::from(value).try_into()?
                    }else{
                        #else_data
                    };
                };

                return Ok(result);
            } else {
                return Ok(
                    quote!(let #struct_field_name = my_http_server::InputParamValue::from(#data_src.get_required(#input_field_name)?).try_into()?;),
                );
            };
        }
        InputFieldSource::Path => {
            panic!("Bug. Should not read from Path at read_not_body part of script");
        }
        InputFieldSource::Header => {
            let input_field_name = input_field.name();
            let input_field_name = input_field_name.as_str();

            let result = quote!(let #struct_field_name = ctx.request.get_required_header(#input_field_name)?.try_into()?;);

            Ok(result)
        }
        InputFieldSource::Body => {
            panic!("Bug. Should not read from Body at read_not_body part of script");
        }
        InputFieldSource::FormData => {
            panic!("Bug. Should not read from FormData at read_not_body part of script");
        }
        InputFieldSource::BodyRaw => {
            panic!("Bug. Should not read from BodyRaw at read_not_body part of script");
        }
    }
}

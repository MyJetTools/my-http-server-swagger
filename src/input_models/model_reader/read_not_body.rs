use std::str::FromStr;

use proc_macro2::TokenStream;
use types_reader::{PropertyType, StructProperty};

use quote::quote;

use crate::{
    as_token_stream::AsTokenStream,
    input_models::{input_model_struct_property_ext::InputModelStructPropertyExt, InputField},
};

pub fn generate_read_not_body(
    properties: &Vec<&StructProperty>,
    read_data_src: impl Fn() -> TokenStream,
) -> Result<TokenStream, syn::Error> {
    let mut validation = Vec::with_capacity(properties.len());
    let data_src = read_data_src();

    let mut reading_fields: Vec<TokenStream> = Vec::with_capacity(properties.len());

    for struct_property in properties {
        let input_field = struct_property.get_input_field()?;
        let struct_field_name = struct_property.get_field_name_ident();

        match &struct_property.ty {
            PropertyType::OptionOf(_) => {
                let input_field_name = input_field.get_input_field_name()?;

                let item = quote! {
                    let #struct_field_name = if let Some(value) = #data_src.get_optional(#input_field_name){
                        let value = my_http_server::InputParamValue::from(value);
                        Some(value.try_into()?)
                    }else{
                        None
                    };
                }.into();

                reading_fields.push(item);
            }
            PropertyType::VecOf(sub_type) => {
                if sub_type.is_string() {
                    let input_field_name = input_field.get_input_field_name()?;

                    let item = quote! {
                      let #struct_field_name = #data_src.get_vec_of_string(#input_field_name)?;
                    }
                    .into();

                    reading_fields.push(item);
                } else {
                    let input_field_name = input_field.get_input_field_name()?;

                    let item = quote! {
                       let #struct_field_name = #data_src.get_vec(#input_field_name)?;
                    }
                    .into();

                    reading_fields.push(item);
                }
            }
            PropertyType::Struct(..) => {
                let input_field_name = input_field.get_input_field_name()?;

                let prop_type = struct_property.get_syn_type();

                let default_value = if let Some(default_value) = input_field.get_default_value()? {
                    match default_value {
                        crate::input_models::DefaultValue::Empty(_) => {
                            Some(quote!(#prop_type::create_default()?))
                        }
                        crate::input_models::DefaultValue::Value(default_value) => Some(
                            quote!(<#prop_type as std::str::FromStr>::from_str(#default_value)?),
                        ),
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

                        reading_fields.push(item);
                    }
                    None => {
                        reading_fields.push(generate_reading_required(struct_property, &data_src)?);
                    }
                }
            }
            _ => {
                reading_fields.push(generate_reading_required(struct_property, &data_src)?);
            }
        }

        if let Some(validator) = input_field.validator()? {
            let validation_fn_name = proc_macro2::TokenStream::from_str(validator).unwrap();
            validation.push(quote!(#validation_fn_name(ctx, &#struct_field_name)?;));
        }
    }

    let init_fields = properties.as_token_stream()?;

    let result = quote! {
        let #init_fields = {
            let #data_src = ctx.request.get_query_string()?;
            #(#reading_fields)*
            #init_fields
        };
        #(#validation)*
    };

    Ok(result)
}

fn generate_reading_required(
    struct_property: &StructProperty,
    data_src: &TokenStream,
) -> Result<TokenStream, syn::Error> {
    let input_field = struct_property.get_input_field()?;
    let struct_field_name = struct_property.get_field_name_ident();
    match input_field {
        InputField::Query(_) => {
            let input_field_name = input_field.get_input_field_name()?;
            if let Some(default_value) = input_field.get_default_value()? {
                match default_value {
                    crate::input_models::DefaultValue::Empty(_) => {
                        let prop_type = struct_property.get_syn_type();
                        let result = quote!(#prop_type::create_default()?);

                        return Ok(result);
                    }
                    crate::input_models::DefaultValue::Value(default) => {
                        let else_data = proc_macro2::TokenStream::from_str(default);

                        if let Err(err) = else_data {
                            return Err(syn::Error::new_spanned(
                                struct_property.field,
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
                    }
                }
            } else {
                return Ok(
                    quote!(let #struct_field_name = my_http_server::InputParamValue::from(#data_src.get_required(#input_field_name)?).try_into()?;),
                );
            };
        }
        InputField::Path(_) => {
            panic!("Bug. Should not read from Path at read_not_body part of script");
        }
        InputField::Header(_) => {
            let input_field_name = input_field.get_input_field_name()?;

            let result = quote!(let #struct_field_name = ctx.request.get_required_header(#input_field_name)?.try_into()?;);

            Ok(result)
        }
        InputField::Body(_) => {
            panic!("Bug. Should not read from Body at read_not_body part of script");
        }
        InputField::FormData(_) => {
            panic!("Bug. Should not read from FormData at read_not_body part of script");
        }
        InputField::BodyRaw(_) => {
            panic!("Bug. Should not read from BodyRaw at read_not_body part of script");
        }
    }
}

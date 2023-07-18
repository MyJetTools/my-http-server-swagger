use std::str::FromStr;

use proc_macro2::TokenStream;
use types_reader::PropertyType;

use quote::quote;

use crate::input_models::InputField;

pub fn generate_read_not_body(
    input_fields: &[InputField],
    read_data_src: impl Fn() -> TokenStream,
) -> Result<TokenStream, syn::Error> {
    let mut validation = Vec::with_capacity(input_fields.len());
    let data_src = read_data_src();

    let mut reading_fields: Vec<TokenStream> = Vec::with_capacity(input_fields.len());

    for input_field in input_fields {
        let struct_field_name = input_field.property.get_field_name_ident();

        match &input_field.property.ty {
            PropertyType::OptionOf(_) => {
                let input_field_name = input_field.get_input_field_name()?;

                let default_value = input_field.get_default_value_non_opt_case()?;

                let item = quote! {
                    let #struct_field_name = if let Some(value) = #data_src.get_optional(#input_field_name){
                        let value = my_http_server::InputParamValue::from(value);
                        Some(value.try_into()?)
                    }else{
                        #default_value
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

                let default_value = input_field.get_default_value_opt_case()?;

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
            _ => {
                reading_fields.push(generate_reading_required(input_field, &data_src)?);
            }
        }

        if let Some(validator) = input_field.validator()? {
            let validation_fn_name = proc_macro2::TokenStream::from_str(validator).unwrap();
            validation.push(quote!(#validation_fn_name(ctx, &#struct_field_name)?;));
        }
    }

    let init_fields = super::utils::get_fields_to_read(input_fields)?;

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
    input_field: &InputField,
    data_src: &TokenStream,
) -> Result<TokenStream, syn::Error> {
    let struct_field_name = input_field.property.get_field_name_ident();
    let input_field_name = input_field.get_input_field_name()?;
    if let Some(default_value) = input_field.get_default_value()? {
        match default_value {
            crate::input_models::DefaultValue::Empty(_) => {
                let prop_type = input_field.property.get_syn_type();
                let result = quote!(#prop_type::create_default()?);

                return Ok(result);
            }
            crate::input_models::DefaultValue::Value(default) => {
                let default = default.get_any_value_as_str()?;
                let else_data = proc_macro2::TokenStream::from_str(default);

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
            }
        }
    } else {
        return Ok(
            quote!(let #struct_field_name = my_http_server::InputParamValue::from(#data_src.get_required(#input_field_name)?).try_into()?;),
        );
    };
}

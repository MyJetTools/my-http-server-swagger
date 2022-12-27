use crate::as_token_stream::AsTokenStream;
use proc_macro2::TokenStream;
use types_reader::PropertyType;

use crate::input_models::input_fields::{InputField, InputFieldSource};

use quote::quote;

pub fn get_query_string_data_src() -> TokenStream {
    quote!(__query_string)
}

pub fn generate_read_not_body(input_fields: &Vec<&InputField>) -> TokenStream {
    let mut validation = Vec::with_capacity(input_fields.len());
    let data_src = get_query_string_data_src();

    let mut reading_feilds = Vec::with_capacity(input_fields.len());

    for input_field in input_fields {
        let struct_field_name = input_field.property.get_field_name_ident();

        match &input_field.property.ty {
            PropertyType::OptionOf(_) => {
                let input_field_name = input_field.name();
                let input_field_name = input_field_name.get_value_as_str();

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
                    let input_field_name = input_field_name.get_value_as_str();

                    let item = quote! {
                      let #struct_field_name = #data_src.get_vec_of_string(#input_field_name)?;
                    }
                    .into();

                    reading_feilds.push(item);
                } else {
                    let input_field_name = input_field.name();
                    let input_field_name = input_field_name.get_value_as_str();

                    let item = quote! {
                       let #struct_field_name = #data_src.get_vec(#input_field_name)?;
                    }
                    .into();

                    reading_feilds.push(item);
                }
            }
            PropertyType::Struct(..) => {
                let input_field_name = input_field.name();
                let input_field_name = input_field_name.get_value_as_str();

                let prop_type = input_field.property.get_syn_type();

                let item = quote! {
                   let #struct_field_name = match #data_src.get_optional(#input_field_name){
                    Some(value) =>{
                        let value = my_http_server::InputParamValue::from(value);
                        value.try_into()?
                    },
                    None => {
                        #prop_type::create_default()?
                    }
                   };

                }
                .into();

                reading_feilds.push(item);
            }
            _ => {
                reading_feilds.push(generate_reading_required(input_field));
            }
        }

        if let Some(validator) = input_field.validator() {
            let validation_fn_name = proc_macro2::Literal::string(validator.get_value_as_str());
            validation.push(quote!(#validation_fn_name(ctx, &#struct_field_name)?;));
        }
    }

    let init_fields = input_fields.as_token_stream();

    quote! {
        let #init_fields = {
            let #data_src = ctx.request.get_query_string()?;
            #(#reading_feilds)*
            #init_fields
        };
        #(#validation)*
    }
}

fn generate_reading_required(input_field: &InputField) -> TokenStream {
    match input_field.src {
        InputFieldSource::Query => {
            let data_src = get_query_string_data_src();
            let input_field_name = input_field.name();
            let input_field_name = input_field_name.get_value_as_str();

            quote!(my_http_server::InputParamValue::from(#data_src.get_required(#input_field_name)?).try_into()?;)
        }
        InputFieldSource::Path => {
            let input_field_name = input_field.name();
            let input_field_name = input_field_name.get_value_as_str();

            quote!(http_route.get_value(&ctx.request.http_path, #input_field_name)?.try_into()?;)
        }
        InputFieldSource::Header => {
            let input_field_name = input_field.name();
            let input_field_name = input_field_name.get_value_as_str();

            quote!(ctx.request.get_required_header(#input_field_name)?.try_into()?;)
        }
        InputFieldSource::Body => {
            panic!("Bug. Should not read Body at generate_reading_required");
        }
        InputFieldSource::FormData => {
            panic!("Bug. Should not read Form at generate_reading_required");
        }
        InputFieldSource::BodyFile => {
            panic!("Bug. Should not read BodyFile at generate_reading_required");
        }
    }
}

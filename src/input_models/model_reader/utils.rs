use proc_macro2::TokenStream;
use types_reader::PropertyType;

use crate::input_models::InputField;

pub fn get_fields_to_read(fields: &[InputField]) -> Result<(TokenStream, TokenStream), syn::Error> {
    if fields.len() == 1 {
        let field = fields.get(0).unwrap();
        let name = field.property.get_field_name_ident();
        let string_transformation = field.get_final_string_transformation()?;
        return Ok((
            quote::quote!(#name),
            quote::quote!(#name #string_transformation),
        ));
    }

    let mut no = 0;

    let mut in_result = Vec::with_capacity(fields.len());
    let mut out_result = Vec::with_capacity(fields.len());

    for input_field in fields {
        if no > 0 {
            in_result.push(quote::quote!(,));
            out_result.push(quote::quote!(,));
        }

        let ident = input_field.property.get_field_name_ident();
        let string_transformation = input_field.get_final_string_transformation()?;
        in_result.push(quote::quote!(#ident));

        out_result.push(quote::quote!(#ident #string_transformation));
        no += 1;
    }

    Ok((
        quote::quote! {(#(#in_result)*)},
        quote::quote! {(#(#out_result)*)},
    ))
}

pub fn verify_default_value(input_field: &InputField, ty: &PropertyType) -> Result<(), syn::Error> {
    let empty_only = match ty {
        PropertyType::U8 => false,
        PropertyType::I8 => false,
        PropertyType::U16 => false,
        PropertyType::I16 => false,
        PropertyType::U32 => false,
        PropertyType::I32 => false,
        PropertyType::U64 => false,
        PropertyType::I64 => false,
        PropertyType::F32 => false,
        PropertyType::F64 => false,
        PropertyType::USize => false,
        PropertyType::ISize => false,
        PropertyType::String => false,
        PropertyType::Str => false,
        PropertyType::Bool => false,
        PropertyType::DateTime => false,
        PropertyType::OptionOf(_) => false,
        PropertyType::VecOf(_) => false,
        PropertyType::Struct(_, _) => true,
        PropertyType::HashMap(_, _) => false,
    };

    if empty_only {
        let default_value = input_field.get_default_value()?;
        match default_value {
            Some(default_value) => {
                if !default_value.is_empty() {
                    return default_value.throw_error("Please use default parameter with NO value");
                }

                return Ok(());
            }
            None => return Ok(()),
        }
    } else {
        let default_value = input_field.get_default_value()?;
        match default_value {
            Some(default_value) => {
                if default_value.is_empty() {
                    return default_value.throw_error("Please use default parameter with value");
                }

                return Ok(());
            }
            None => return Ok(()),
        }
    }
}

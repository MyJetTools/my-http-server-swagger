use std::str::FromStr;

use proc_macro2::TokenStream;

use super::input_fields::{InputField, InputFieldSource, InputFields};
use quote::quote;

pub fn generate_http_input(fields: &InputFields) -> Result<TokenStream, syn::Error> {
    let mut doc_fields = Vec::new();
    for input_field in &fields.fields {
        doc_fields.push(generate_http_input_parameter(input_field)?);
    }

    let use_documentation = crate::consts::get_use_documentation();
    let result = quote! {
        #use_documentation;
        vec![#(#doc_fields),*]
    };

    Ok(result)
}

fn generate_http_input_parameter(input_field: &InputField) -> Result<TokenStream, syn::Error> {
    let field = if input_field.src_is_body() {
        if let Some(body_type) = input_field.get_body_type() {
            let body_type = body_type.get_value_as_str();

            crate::types::compile_http_field_with_object(
                input_field.name().get_value_as_str(),
                &body_type,
                input_field.required(),
                input_field.get_default_value(),
            )
        } else {
            crate::types::compile_http_field(
                input_field.name().get_value_as_str(),
                &input_field.property.ty,
                input_field.required(),
                input_field.get_default_value(),
                Some(&input_field.src),
            )
        }
    } else {
        crate::types::compile_http_field(
            input_field.name().get_value_as_str(),
            &input_field.property.ty,
            input_field.required(),
            input_field.get_default_value(),
            Some(&input_field.src),
        )
    };

    let http_input_parameter_type = crate::consts::get_http_input_parameter_type();
    let description = input_field.description()?;
    let description = description.get_value_as_str();

    let source = get_input_src(input_field);

    let result = quote! {
        #http_input_parameter_type{
            field: #field,
            description: #description.to_string(),
            source: #source
        }
    };

    Ok(result)
}

fn get_input_src(field: &InputField) -> TokenStream {
    let http_parameter_input_src = crate::consts::get_http_parameter_input_src();

    match field.src {
        InputFieldSource::Query => quote!(#http_parameter_input_src::Query),
        InputFieldSource::Path => quote!(#http_parameter_input_src::Path),
        InputFieldSource::Header => quote!(#http_parameter_input_src::Header),
        InputFieldSource::Body => quote!(#http_parameter_input_src::Body),
        InputFieldSource::FormData => quote!(#http_parameter_input_src::FormData),
    }
}

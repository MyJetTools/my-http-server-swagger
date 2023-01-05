use proc_macro2::TokenStream;

use super::input_fields::{InputField, InputFieldSource, InputFields};
use quote::quote;

pub fn generate_http_input(fields: &InputFields) -> Result<TokenStream, syn::Error> {
    let mut doc_fields = Vec::new();
    for input_field in &fields.fields {
        match generate_http_input_parameter(input_field) {
            Ok(field) => doc_fields.push(field),
            Err(e) => doc_fields.push(e.to_compile_error().into()),
        }
    }

    let use_documentation = crate::consts::get_use_documentation();
    let result = quote! {
        #use_documentation;
        vec![#(#doc_fields),*]
    };

    Ok(result)
}

fn generate_http_input_parameter(input_field: &InputField) -> Result<TokenStream, syn::Error> {
    let field = crate::types::compile_http_field(
        input_field.name().as_str(),
        &input_field.property.ty,
        input_field.required(),
        input_field.get_default_value(),
    );

    let http_input_parameter_type = crate::consts::get_http_input_parameter();
    let description = input_field.description()?;
    let description = description.as_str();

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
        InputFieldSource::BodyRaw => quote!(#http_parameter_input_src::Body),
    }
}

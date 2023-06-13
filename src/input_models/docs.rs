use proc_macro2::TokenStream;
use types_reader::StructProperty;

use quote::quote;

use super::{input_model_struct_property_ext::InputModelStructPropertyExt, InputField};

pub fn generate_http_input(
    properties: &[StructProperty],
    has_generic_type_as_param: bool,
) -> Result<TokenStream, syn::Error> {
    let mut doc_fields = Vec::new();
    for struct_property in properties {
        let input_field = struct_property.get_input_field()?;
        match generate_http_input_parameter(&input_field, has_generic_type_as_param) {
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

fn generate_http_input_parameter(
    input_field: &InputField,
    has_generic_type_as_param: bool,
) -> Result<TokenStream, syn::Error> {
    let input_field_data = input_field.get_input_data();

    let field = crate::types::compile_http_field(
        input_field_data.get_input_field_name()?,
        &input_field_data.property.ty,
        input_field_data.get_default_value()?,
        has_generic_type_as_param,
    )?;

    let http_input_parameter_type = crate::consts::get_http_input_parameter();
    let description = input_field_data.get_description()?;

    let source = input_field.get_input_src_token();

    let result = quote! {
        #http_input_parameter_type{
            field: #field,
            description: #description.to_string(),
            source: #source
        }
    };

    Ok(result)
}

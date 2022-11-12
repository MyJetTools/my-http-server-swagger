use crate::consts::{HTTP_INPUT_PARAMETER_TYPE, HTTP_PARAMETER_INPUT_SRC, USE_DOCUMENTATION};

use super::input_fields::{InputField, InputFieldSource, InputFields};

pub fn generate_http_input(result: &mut String, fields: &InputFields) {
    result.push_str(USE_DOCUMENTATION);

    result.push_str("vec![");
    for input_field in &fields.fields {
        generate_http_input_parameter(result, input_field);
    }

    result.push(']');
}

fn generate_http_input_parameter(result: &mut String, input_field: &InputField) {
    result.push_str(HTTP_INPUT_PARAMETER_TYPE);

    result.push_str("{ field: ");

    if input_field.src_is_body() {
        if let Some(body_type) = input_field.my_attr.get_as_string("body_type") {
            crate::types::compile_http_field_with_object(
                result,
                input_field.name(),
                body_type,
                input_field.required(),
                input_field.get_default_value(),
            );
        } else {
            crate::types::compile_http_field(
                result,
                input_field.name(),
                &input_field.property.ty,
                input_field.required(),
                input_field.get_default_value(),
                Some(&input_field.src),
            );
        }
    } else {
        crate::types::compile_http_field(
            result,
            input_field.name(),
            &input_field.property.ty,
            input_field.required(),
            input_field.get_default_value(),
            Some(&input_field.src),
        );
    };

    result.push_str(", description: \"");
    result.push_str(input_field.description());

    result.push_str("\".to_string(), source: ");
    get_input_src(result, input_field);
    result.push_str("},");
}

fn get_input_src(result: &mut String, field: &InputField) {
    result.push_str(HTTP_PARAMETER_INPUT_SRC);

    let field = match field.src {
        InputFieldSource::Query => "::Query",
        InputFieldSource::Path => "::Path",
        InputFieldSource::Header => "::Header",
        InputFieldSource::Body => "::Body",
        InputFieldSource::FormData => "::FormData",
        InputFieldSource::BodyFile => "::Body",
    };

    result.push_str(field);
}

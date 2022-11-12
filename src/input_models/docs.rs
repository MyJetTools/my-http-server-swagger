use crate::consts::{HTTP_INPUT_PARAMETER_TYPE, HTTP_PARAMETER_INPUT_SRC, USE_DOCUMENTATION};

use super::input_fields::{InputField, InputFieldSource, InputFields};

pub fn generate_http_input(result: &mut String, fields: &InputFields) {
    result.push_str(USE_DOCUMENTATION);

    result.push_str("vec![");
    for input_field in &fields.fields {
        let itm = generate_http_input_parameter(input_field);
        result.push_str(itm.as_str());
    }

    result.push(']');
}

fn generate_http_input_parameter(input_field: &InputField) -> String {
    let http_field = if input_field.src_is_body() {
        if let Some(body_type) = input_field.my_attr.get_as_string("body_type") {
            crate::types::compile_http_field_with_object(
                input_field.name(),
                body_type,
                input_field.required(),
                input_field.get_default_value(),
            )
        } else {
            crate::types::compile_http_field(
                input_field.name(),
                &input_field.property.ty,
                input_field.required(),
                input_field.get_default_value(),
                Some(&input_field.src),
            )
        }
    } else {
        crate::types::compile_http_field(
            input_field.name(),
            &input_field.property.ty,
            input_field.required(),
            input_field.get_default_value(),
            Some(&input_field.src),
        )
    };

    format!(
        r###"{HTTP_INPUT_PARAMETER_TYPE}{{
                    field: {http_field},
                    description: "{description}".to_string(),
                    source: {source},
                }},"###,
        http_field = http_field,
        description = input_field.description(),
        source = get_input_src(input_field)
    )
}

fn get_input_src(field: &InputField) -> String {
    let field = match field.src {
        InputFieldSource::Query => "Query",
        InputFieldSource::Path => "Path",
        InputFieldSource::Header => "Header",
        InputFieldSource::Body => "Body",
        InputFieldSource::FormData => "FormData",
        InputFieldSource::BodyFile => "Body",
    };

    return format!("{HTTP_PARAMETER_INPUT_SRC}::{field}",);
}

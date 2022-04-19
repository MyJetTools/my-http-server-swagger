use crate::{input_models::input_fields::InputField, reflection::PropertyType};

use super::consts::DATA_SOURCE;

pub fn as_string(result: &mut String, input_field: &InputField, default: &str) {
    generate_read_optional_from_query_string_first_line(result, input_field);
    super::extensions::as_string(result);

    result.push_str("}else{\"");
    result.push_str(default);
    result.push_str("\".to_string()}");
}

pub fn parse_as_type(
    result: &mut String,
    input_field: &InputField,
    generic_type: &PropertyType,
    default: &str,
) {
    generate_read_optional_from_query_string_first_line(result, input_field);
    super::extensions::parse_as_type(result, generic_type);

    result.push_str("}else{\"");
    result.push_str(default);
    result.push_str("\"");
    super::extensions::parse_as_type(result, generic_type);
    result.push_str("}");
}

pub fn as_bool(result: &mut String, input_field: &InputField, default: &str) {
    generate_read_optional_from_query_string_first_line(result, input_field);
    super::extensions::as_bool(result);
    result.push_str("}else{");
    result.push_str(default);
    result.push_str("}");
}

pub fn as_simple_type(
    result: &mut String,
    input_field: &InputField,
    generic_type: &PropertyType,
    default: &str,
) {
    generate_read_optional_from_query_string_first_line(result, input_field);
    super::extensions::parse_as_type(result, generic_type);
    result.push_str(")}else{");
    result.push_str(default);
    result.push_str("}");
}

fn generate_read_optional_from_query_string_first_line(
    result: &mut String,
    input_field: &InputField,
) {
    result.push_str("if let Some(value) = ");
    result.push_str(DATA_SOURCE);
    result.push_str(".get_optional(\"");
    result.push_str(input_field.name());
    result.push_str("\"){value");
}

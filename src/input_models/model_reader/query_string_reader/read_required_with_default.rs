use crate::{input_models::input_fields::InputField, reflection::PropertyType};

use super::SourceToRead;

pub fn as_string(
    result: &mut String,
    source_to_read: &SourceToRead,
    input_field: &InputField,
    default: &str,
) {
    generate_read_optional_from_query_string_first_line(result, source_to_read, input_field);
    super::extensions::as_string(result);

    result.push_str("}else{\"");
    result.push_str(default);
    result.push_str("\".to_string()}");
}

pub fn parse_as_type(
    result: &mut String,
    source_to_read: &SourceToRead,
    input_field: &InputField,
    generic_type: &PropertyType,
    default: &str,
) {
    generate_read_optional_from_query_string_first_line(result, source_to_read, input_field);
    super::extensions::parse_as_type(result, generic_type);

    result.push_str("}else{\"");
    result.push_str(default);
    result.push_str("\"");
    super::extensions::parse_as_type(result, generic_type);
    result.push_str("}");
}

pub fn as_bool(
    result: &mut String,
    source_to_read: &SourceToRead,
    input_field: &InputField,

    default: &str,
) {
    generate_read_optional_from_query_string_first_line(result, source_to_read, input_field);
    super::extensions::as_bool(result);
    result.push_str("}else{");
    result.push_str(default);
    result.push_str("}");
}

pub fn as_simple_type(
    result: &mut String,
    source_to_read: &SourceToRead,
    input_field: &InputField,
    generic_type: &PropertyType,
    default: &str,
) {
    generate_read_optional_from_query_string_first_line(result, source_to_read, input_field);
    super::extensions::parse_as_type(result, generic_type);
    result.push_str(")}else{");
    result.push_str(default);
    result.push_str("}");
}

fn generate_read_optional_from_query_string_first_line(
    result: &mut String,
    source_to_read: &SourceToRead,
    input_field: &InputField,
) {
    result.push_str("if let Some(value) =");
    result.push_str(source_to_read.get_source_variable());
    result.push_str(".get_optional(\"");
    result.push_str(input_field.name());
    result.push_str("\"){value");
}

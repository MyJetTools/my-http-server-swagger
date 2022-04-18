use crate::input_models::input_fields::InputField;

use super::SourceToRead;

pub fn as_string(result: &mut String, source_to_read: &SourceToRead, input_field: &InputField) {
    generate_read_optional_from_query_string_first_line(result, source_to_read, input_field);
    super::extensions::as_string(result);
    generate_read_optional_from_query_string_second_line(result);
}

pub fn as_bool(result: &mut String, source_to_read: &SourceToRead, input_field: &InputField) {
    generate_read_optional_from_query_string_first_line(result, source_to_read, input_field);
    super::extensions::as_bool(result);
    generate_read_optional_from_query_string_second_line(result);
}

pub fn parase_as_type(
    result: &mut String,
    source_to_read: &SourceToRead,
    input_field: &InputField,
) {
    generate_read_optional_from_query_string_first_line(result, source_to_read, input_field);
    super::extensions::parse_as_type(result, input_field);
    generate_read_optional_from_query_string_second_line(result);
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
    result.push_str("\"){Some(value");
}

fn generate_read_optional_from_query_string_second_line(result: &mut String) {
    result.push_str(")}else{None}");
}

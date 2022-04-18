use crate::input_models::input_fields::InputField;

use super::SourceToRead;

pub fn as_string(result: &mut String, source_to_read: &SourceToRead, input_field: &InputField) {
    generate_read_required_from_query_string(result, source_to_read, input_field);
    super::extensions::as_string(result);
}

pub fn parse_as_type(result: &mut String, source_to_read: &SourceToRead, input_field: &InputField) {
    generate_read_required_from_query_string(result, source_to_read, input_field);
    super::extensions::parse_as_type(result, &input_field.property.ty);
}

pub fn as_bool(result: &mut String, source_to_read: &SourceToRead, input_field: &InputField) {
    generate_read_required_from_query_string(result, source_to_read, input_field);
    super::extensions::as_bool(result);
}

fn generate_read_required_from_query_string(
    result: &mut String,
    source_to_read: &SourceToRead,
    input_field: &InputField,
) {
    result.push_str(source_to_read.get_source_variable());
    result.push_str(".get_required(\"");
    result.push_str(input_field.name());
    result.push_str("\")?");
}

use crate::input_models::input_fields::InputField;

use super::consts::DATA_SOURCE;

pub fn as_string(result: &mut String, input_field: &InputField) {
    generate_read_required_from_query_string(result, input_field);
    super::extensions::as_string(result);
}

pub fn parse_as_type(result: &mut String, input_field: &InputField) {
    generate_read_required_from_query_string(result, input_field);
    super::extensions::parse_as_type(result, &input_field.property.ty);
}

pub fn as_bool(result: &mut String, input_field: &InputField) {
    generate_read_required_from_query_string(result, input_field);
    super::extensions::as_bool(result);
}

fn generate_read_required_from_query_string(result: &mut String, input_field: &InputField) {
    result.push_str(DATA_SOURCE);
    result.push_str(".get_required(\"");
    result.push_str(input_field.name());
    result.push_str("\")?");
}

use crate::input_models::input_fields::InputField;

pub fn as_string(result: &mut String) {
    result.push_str(".as_string()?");
}

pub fn as_bool(result: &mut String) {
    result.push_str(".as_bool()?");
}

pub fn parse_as_type(result: &mut String, input_field: &InputField) {
    result.push_str(".parse::<");
    result.push_str(input_field.property.ty.as_str().as_str());
    result.push_str(">()?");
}

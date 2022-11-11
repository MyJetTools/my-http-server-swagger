use crate::reflection::PropertyType;

pub fn as_string(result: &mut String) {
    result.push_str(".as_string()?");
}

pub fn as_bool(result: &mut String) {
    result.push_str(".as_bool()?");
}

pub fn as_date_time(result: &mut String) {
    result.push_str(".as_date_time()?");
}

pub fn parse_as_type(result: &mut String, ty: &PropertyType) {
    result.push_str(".parse::<");
    result.push_str(ty.as_str().as_str());
    result.push_str(">()?");
}

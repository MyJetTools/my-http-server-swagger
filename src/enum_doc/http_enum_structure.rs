use super::enum_json::EnumJson;
use crate::consts::{
    ENUM_TYPE, HTTP_ENUM_CASE, HTTP_ENUM_STRUCTURE, NAME_SPACE, USE_DOCUMENTATION,
};

pub fn generate(name: &str, is_string: bool, enum_cases: &[EnumJson]) -> String {
    let mut result = String::new();

    result.push_str(USE_DOCUMENTATION);

    result.push_str(format!("{NAME_SPACE}::{HTTP_ENUM_STRUCTURE} {{").as_str());
    result.push_str(format!("struct_id: \"{}\".to_string(),", name).as_str());

    let tp = if is_string { "String" } else { "Integer" };

    result.push_str(format!("enum_type: {NAME_SPACE}::{ENUM_TYPE}::{tp},", tp = tp).as_str());

    result.push_str("cases: vec![");

    for enum_json in enum_cases {
        if let Some(data_to_add) = compile_enum_case(enum_json) {
            result.push_str(data_to_add.as_str());
            result.push(',');
        } else {
        }
    }
    result.push_str("],}");

    result
}

fn compile_enum_case(enum_case: &EnumJson) -> Option<String> {
    format!(
        "{NAME_SPACE}::{HTTP_ENUM_CASE}{{id:{the_id}, value:\"{value}\".to_string(), description:\"{description}\".to_string()}}",
        the_id = enum_case.get_id(),
        value = enum_case.get_value(),
        description = enum_case.description()
    )
    .into()
}

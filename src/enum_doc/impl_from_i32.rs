use super::enum_json::EnumJson;

pub fn generate(enum_cases: &[EnumJson]) -> String {
    let mut result = String::new();

    result.push_str("match src {");

    let mut default = None;

    for enum_case in enum_cases {
        if enum_case.has_default_attr() {
            default = Some(enum_case.get_enum_case_value());
            continue;
        }
        result.push_str(enum_case.get_id().to_string().as_str());
        result.push_str(" => Self::");
        result.push_str(enum_case.get_enum_case_value());
        result.push(',');
    }

    if let Some(default) = default {
        result.push_str("_ => Self::");
        result.push_str(default);
    } else {
        result.push_str("_ => panic!(\"Can not parse enum with value {}\", src)");
    }

    result.push_str("}");

    result
}

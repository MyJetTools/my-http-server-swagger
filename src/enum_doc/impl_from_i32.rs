use super::enum_json::EnumJson;

pub fn generate(result: &mut String, enum_cases: &[EnumJson]) {
    result.push_str("match src {");

    let mut default = false;

    for enum_case in enum_cases {
        if enum_case.has_default_attr() {
            default = true;
            continue;
        }
        result.push_str(enum_case.get_id().to_string().as_str());
        result.push_str(" => Self::");
        result.push_str(enum_case.get_enum_case_value());
        result.push(',');
    }

    if default {
        result.push_str("_ => Self::default()");
    } else {
        result.push_str("_ => panic!(\"Can not parse enum with value {}\", src)");
    }

    result.push_str("}");
}

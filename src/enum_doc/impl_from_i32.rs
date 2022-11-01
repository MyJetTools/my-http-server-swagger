use super::enum_json::EnumJson;

pub fn generate(enum_cases: &[EnumJson]) -> String {
    let mut result = String::new();

    result.push_str("match src {");

    for enum_case in enum_cases {
        result.push_str(enum_case.get_id().to_string().as_str());
        result.push_str(" => Self::");
        result.push_str(enum_case.get_enum_case_value());
        result.push(',');
    }

    result.push_str("}");

    result
}

use super::enum_json::EnumJson;

const HTTP_FAIL_RESULT: &str = "my_http_server::HttpFailResult";

pub fn generate(result: &mut String, name: &str, enum_cases: &[EnumJson]) {
    result.push_str("type Err = ");
    result.push_str(HTTP_FAIL_RESULT);
    result.push_str("; fn from_str(src:&str)->Result<Self,Self::Err>{");
    generate_content(result, name, enum_cases);
    result.push('}');
}

fn generate_content(result: &mut String, name: &str, enum_cases: &[EnumJson]) {
    let mut default_value = false;
    for enum_case in enum_cases {
        if enum_case.is_default_value {
            default_value = true;
        }

        let line_to_add = format!(
            "if src == \"{value}\" || src == \"{the_id}\"{{return Ok(Self::{enum_value})}}\n",
            value = enum_case.get_value(),
            the_id = enum_case.get_id(),
            enum_value = enum_case.get_enum_case_value(),
        );

        result.push_str(line_to_add.as_str());
    }

    if default_value {
        result.push_str("Ok(Self::default())");
    } else {
        let line_to_add = format!(
            "Err({http_fail_result}::as_forbidden(Some(\"{err}\".to_string())))",
            http_fail_result = HTTP_FAIL_RESULT,
            err = format!("Can not parse {} enum", name)
        );
        result.push_str(line_to_add.as_str());
    }
}

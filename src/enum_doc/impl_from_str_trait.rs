use super::enum_json::EnumJson;

const HTTP_FAIL_RESULT: &str = "my_http_server::HttpFailResult";

pub fn generate(name: &str, enum_cases: &[EnumJson]) -> String {
    format!(
        r###"impl std::str::FromStr for {name} {{
               type Err = {http_fail_result};
               fn from_str(src: &str) -> Result<Self, Self::Err> {{{content}}}
            }}"###,
        content = generate_content(name, enum_cases),
        http_fail_result = HTTP_FAIL_RESULT,
    )
}

fn generate_content(name: &str, enum_cases: &[EnumJson]) -> String {
    let mut result = String::new();

    let mut default_value = None;
    for enum_case in enum_cases {
        if enum_case.has_default_attr() {
            default_value = Some(enum_case.get_enum_case_value());
        }

        let line_to_add = format!(
            "if src == \"{value}\" || src == \"{the_id}\"{{return Ok(Self::{enum_value})}}\n",
            value = enum_case.get_value(),
            the_id = enum_case.get_id(),
            enum_value = enum_case.get_enum_case_value(),
        );

        result.push_str(line_to_add.as_str());
    }

    if let Some(default) = default_value {
        let line_to_add = format!("Ok(Self::{})", default);
        result.push_str(line_to_add.as_str());
    } else {
        let line_to_add = format!(
            "Err({http_fail_result}::as_forbidden(Some(\"{err}\".to_string())))",
            http_fail_result = HTTP_FAIL_RESULT,
            err = format!("Can not parse {} enum", name)
        );
        result.push_str(line_to_add.as_str());
    }

    result
}

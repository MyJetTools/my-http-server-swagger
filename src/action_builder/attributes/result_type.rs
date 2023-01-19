pub enum ResultType {
    Object(String),
    Array(String),
    ArrayOfSimpleType(String),
    SimpleType(String),
}

impl ResultType {
    pub fn new(model: Option<String>, model_as_array: Option<String>) -> Option<Self> {
        if let Some(model_as_object) = model {
            if let Some(vec_model) = is_model_vec(&model_as_object) {
                if is_simple_type(vec_model) {
                    return ResultType::ArrayOfSimpleType(vec_model.to_string()).into();
                }

                return ResultType::Array(vec_model.to_string()).into();
            }

            if is_simple_type(model_as_object.as_str()) {
                return ResultType::SimpleType(model_as_object).into();
            }

            return ResultType::Object(model_as_object).into();
        }

        if let Some(model_as_array) = model_as_array {
            if is_simple_type(model_as_array.as_str()) {
                return ResultType::ArrayOfSimpleType(model_as_array).into();
            }

            return ResultType::Array(model_as_array).into();
        }

        None
    }
}

fn is_simple_type(src: &str) -> bool {
    match src {
        "String" => true,
        "Integer" => true,
        "Long" => true,
        "Float" => true,
        "Double" => true,
        "Byte" => true,
        "Binary" => false,
        "Boolean" => true,
        "Date" => true,
        "DateTime" => true,
        "Password" => true,
        _ => false,
    }
}

fn is_model_vec(model_as_string: &str) -> Option<&str> {
    if model_as_string.starts_with("Vec<") {
        return Some(&model_as_string[4..model_as_string.len() - 1]);
    }

    None
}

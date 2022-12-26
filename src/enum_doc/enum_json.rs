use macros_utils::ParamValue;

use crate::reflection::EnumCase;

pub struct EnumJson {
    src: EnumCase,
    pub is_default_value: bool,
}

pub const HTTP_ENUM_ATTR_NAME: &str = "http_enum_case";

impl EnumJson {
    pub fn new(src: EnumCase) -> Option<Self> {
        if let Some(value) = src.attrs.get(HTTP_ENUM_ATTR_NAME) {
            if let Some(value) = value {
                let is_default_value = value.has_param("default");
                return Self {
                    src,
                    is_default_value,
                }
                .into();
            }
        }

        return None;
    }

    pub fn get_id(&self) -> isize {
        if let Some(value) = self.src.attrs.get(HTTP_ENUM_ATTR_NAME) {
            if let Some(value) = value {
                if let Some(id) = value.get_named_param("id") {
                    return id.get_value();
                }
            }
        }

        panic!("[id] is not found for the field {}", self.src.name);
    }

    pub fn get_enum_case_value(&self) -> &str {
        self.src.name.as_str()
    }

    pub fn get_value(&self) -> ParamValue {
        if let Some(value) = self.src.attrs.get(HTTP_ENUM_ATTR_NAME) {
            if let Some(value) = value {
                if let Some(id) = value.get_named_param("value") {
                    return id;
                }
            }
        }

        ParamValue {
            value: self.src.name.as_bytes(),
        }
    }

    pub fn description(&self) -> ParamValue {
        if let Some(value) = self.src.attrs.get(HTTP_ENUM_ATTR_NAME) {
            if let Some(value) = value {
                if let Some(id) = value.get_named_param("description") {
                    return id;
                }
            }
        }

        panic!("[description] is not found for the field {}", self.src.name);
    }
}

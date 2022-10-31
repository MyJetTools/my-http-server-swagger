use crate::reflection::EnumCase;

pub struct EnumJson {
    src: EnumCase,
}

pub const HTTP_ENUM_ATTR_NAME: &str = "http_enum_case";

impl EnumJson {
    pub fn new(src: EnumCase) -> Option<Self> {
        if !src.attrs.has_attr(HTTP_ENUM_ATTR_NAME) {
            return None;
        }

        Self { src }.into()
    }

    pub fn has_default_attr(&self) -> bool {
        self.src.attrs.get(HTTP_ENUM_ATTR_NAME).has_attr("default")
    }

    pub fn id(&self) -> &str {
        if let Some(value) = self.src.attrs.get(HTTP_ENUM_ATTR_NAME).get_as_string("id") {
            return value;
        }

        panic!("[id] is not found for the field {}", self.src.name);
    }

    pub fn get_enum_case_value(&self) -> &str {
        self.src.name.as_str()
    }

    pub fn get_value(&self) -> &str {
        match self
            .src
            .attrs
            .get(HTTP_ENUM_ATTR_NAME)
            .get_as_string("value")
        {
            Some(value) => value,
            None => self.src.name.as_str(),
        }
    }

    pub fn description(&self) -> &str {
        if let Some(value) = self
            .src
            .attrs
            .get(HTTP_ENUM_ATTR_NAME)
            .get_as_string("description")
        {
            return value;
        };

        panic!("[description] is not found for the field {}", self.src.name);
    }
}

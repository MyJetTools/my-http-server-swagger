use crate::reflection::{EnumCase, MyAttribute};

pub struct EnumJson {
    src: EnumCase,
}

pub const HTTP_ENUM_ATTR_NAME: &str = "http_enum_case";

impl EnumJson {
    pub fn new(src: EnumCase) -> Option<Self> {
        if !src.attrs.contains_key(HTTP_ENUM_ATTR_NAME) {
            return None;
        }

        Self { src }.into()
    }

    fn get_the_attr(&self) -> &MyAttribute {
        self.src.attrs.get(HTTP_ENUM_ATTR_NAME).unwrap()
    }

    pub fn id(&self) -> &str {
        if let Some(value) = self.get_the_attr().get_value("id") {
            return value;
        }

        panic!("[id] is not found for the field {}", self.src.name);
    }

    pub fn get_value(&self) -> &str {
        match self.get_the_attr().get_value("value") {
            Some(value) => value,
            None => self.src.name.as_str(),
        }
    }

    pub fn description(&self) -> &str {
        if let Some(value) = self.get_the_attr().get_value("description") {
            return value;
        };

        panic!("[description] is not found for the field {}", self.src.name);
    }
}

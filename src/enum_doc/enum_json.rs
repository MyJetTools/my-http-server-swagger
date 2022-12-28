use macros_utils::ParamValue;
use types_reader::EnumCase;

pub struct EnumJson<'s> {
    pub src: EnumCase<'s>,
    pub is_default_value: bool,
}

pub const HTTP_ENUM_ATTR_NAME: &str = "http_enum_case";

impl<'s> EnumJson<'s> {
    pub fn new(src: EnumCase<'s>) -> Option<Self> {
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

    pub fn get_id(&self) -> Result<isize, syn::Error> {
        if let Some(value) = self.src.attrs.get(HTTP_ENUM_ATTR_NAME) {
            if let Some(value) = value {
                if let Some(id) = value.get_named_param("id") {
                    return Ok(id.get_value());
                }
            }
        }

        let err = syn::Error::new_spanned(self.src.variant, "[id] is not found");
        Err(err)
    }

    pub fn get_enum_case_value(&self) -> &str {
        &self.src.name
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

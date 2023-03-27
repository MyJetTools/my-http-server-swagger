use types_reader::{attribute_params::ParamValue, EnumCase};

pub struct EnumJson<'s> {
    pub src: EnumCase<'s>,
    pub is_default_value: bool,
}

pub const HTTP_ENUM_ATTR_NAME: &str = "http_enum_case";

impl<'s> EnumJson<'s> {
    pub fn new(src: EnumCase<'s>) -> Option<Self> {
        if let Ok(value) = src.attrs.get_attr(HTTP_ENUM_ATTR_NAME) {
            let is_default_value = value.has_param("default");
            return Self {
                src,
                is_default_value,
            }
            .into();
        }

        return None;
    }

    pub fn get_id(&self) -> Result<isize, syn::Error> {
        if let Ok(value) = self.src.attrs.get_named_param(HTTP_ENUM_ATTR_NAME, "id") {
            return Ok(value.get_value("Value must be a number".into())?);
        }

        let err = syn::Error::new_spanned(self.src.get_name_ident(), "[id] is not found");
        Err(err)
    }

    pub fn get_enum_case_value(&self) -> String {
        self.src.get_name_ident().to_string()
    }

    pub fn get_value(&self) -> String {
        if let Ok(value) = self.src.attrs.get_named_param(HTTP_ENUM_ATTR_NAME, "value") {
            return value.as_str().to_string();
        }

        self.src.get_name_ident().to_string()
    }

    pub fn description(&self) -> Result<ParamValue, syn::Error> {
        self.src
            .attrs
            .get_named_param(HTTP_ENUM_ATTR_NAME, "description")
    }
}

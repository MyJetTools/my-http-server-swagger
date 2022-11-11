use macros_utils::attributes::AttributeFields;

use crate::reflection::StructProperty;

pub enum InputFieldSource {
    Query,
    Path,
    Header,
    Body,
    Form,
    BodyFile,
}

impl InputFieldSource {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            "http_query" => Some(Self::Query),
            "http_header" => Some(Self::Header),
            "http_path" => Some(Self::Path),
            "http_form" => Some(Self::Form),
            "http_body" => Some(Self::Body),
            "http_body_file" => Some(Self::BodyFile),
            _ => None,
        }
    }

    pub fn is_body_file(&self) -> bool {
        match self {
            InputFieldSource::BodyFile => true,
            _ => false,
        }
    }
}

pub struct InputField {
    pub property: StructProperty,
    pub src: InputFieldSource,
    pub my_attr: AttributeFields,
}

fn get_attr(property: &StructProperty) -> Option<(AttributeFields, InputFieldSource)> {
    for (name, fields) in &property.attrs.data {
        let src = InputFieldSource::from_str(name.as_str());

        if let Some(src) = src {
            return Some((fields.clone(), src));
        }
    }
    None
}

impl InputField {
    pub fn new(property: StructProperty) -> Option<Self> {
        let (my_attr, src) = get_attr(&property)?;

        return Self {
            property,
            src,
            my_attr,
        }
        .into();
    }

    pub fn name(&self) -> &str {
        if let Some(value) = self.my_attr.get_as_string("name") {
            value
        } else {
            self.property.name.as_str()
        }
    }

    pub fn required(&self) -> bool {
        if self.property.ty.is_vec() {
            return false;
        }

        !self.property.ty.is_option()
    }

    pub fn get_default_value(&self) -> Option<&str> {
        self.my_attr.get_as_string("default")
    }

    pub fn is_reading_from_body(&self) -> bool {
        match self.src {
            InputFieldSource::Query => false,
            InputFieldSource::Path => false,
            InputFieldSource::Header => false,
            InputFieldSource::Body => true,
            InputFieldSource::Form => true,
            InputFieldSource::BodyFile => true,
        }
    }

    pub fn is_body(&self) -> bool {
        if let InputFieldSource::Body = self.src {
            return true;
        }

        return false;
    }

    pub fn description(&self) -> &str {
        if let Some(value) = self.my_attr.get_as_string("description") {
            return value;
        }

        panic!(
            "Description field is missing of the field {}",
            self.property.name
        );
    }

    pub fn validator(&self) -> Option<&str> {
        self.my_attr.get_as_string("validator")
    }

    pub fn struct_field_name(&self) -> &str {
        self.property.name.as_str()
    }
}

pub struct InputFields {
    pub fields: Vec<InputField>,
}

impl InputFields {
    pub fn new(src: Vec<StructProperty>) -> Self {
        let mut fields = Vec::new();

        for prop in src {
            if let Some(field) = InputField::new(prop) {
                fields.push(field);
            }
        }

        Self { fields }
    }

    pub fn has_query(&self) -> bool {
        for field in &self.fields {
            if let InputFieldSource::Query = &field.src {
                return true;
            }
        }
        return false;
    }

    pub fn has_form_data(&self) -> bool {
        for field in &self.fields {
            if let InputFieldSource::Form = &field.src {
                return true;
            }
        }
        return false;
    }

    pub fn get_from_header_elements(&self) -> Vec<&InputField> {
        let mut result = Vec::new();
        for field in &self.fields {
            if let InputFieldSource::Header = &field.src {
                result.push(field);
            }
        }
        return result;
    }
}

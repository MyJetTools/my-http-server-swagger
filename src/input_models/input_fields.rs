use macros_utils::attributes::AttributeFields;

use crate::reflection::StructProperty;

pub enum BodyDataToReader {
    FormData,
    BodyFile,
    RawBodyToVec,
    DeserializeBody,
    BodyModel,
}

pub enum InputFieldSource {
    Query,
    Path,
    Header,
    Body,
    FormData,
    BodyFile,
}

impl InputFieldSource {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            "http_query" => Some(Self::Query),
            "http_header" => Some(Self::Header),
            "http_path" => Some(Self::Path),
            "http_form_data" => Some(Self::FormData),
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

    pub fn is_body_to_vec(&self) -> bool {
        self.property.ty.is_vec() && self.src_is_body()
    }

    pub fn is_reading_from_body(&self) -> bool {
        match self.src {
            InputFieldSource::Query => false,
            InputFieldSource::Path => false,
            InputFieldSource::Header => false,
            InputFieldSource::Body => true,
            InputFieldSource::FormData => true,
            InputFieldSource::BodyFile => true,
        }
    }

    pub fn src_is_body(&self) -> bool {
        if let InputFieldSource::Body = self.src {
            return true;
        }

        return false;
    }

    pub fn src_is_form_data(&self) -> bool {
        if let InputFieldSource::FormData = self.src {
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

    pub fn check_types_of_field(&self) {
        let mut has_body_file = 0;
        let mut has_body = 0;
        let mut has_form = 0;

        for field in &self.fields {
            match field.src {
                InputFieldSource::Query => {}
                InputFieldSource::Path => {}
                InputFieldSource::Header => {}
                InputFieldSource::Body => has_body += 1,
                InputFieldSource::FormData => has_form += 1,
                InputFieldSource::BodyFile => has_body_file += 1,
            }
        }

        if has_body_file > 1 {
            panic!("Only one field can be attributed as body_file");
        }

        if has_body_file > 0 && has_body > 0 {
            panic!("Model can not have both body_file attribute and body attribute");
        }

        if has_body_file > 0 && has_form > 0 {
            panic!("Model can not have both body_file attribute and from attribute");
        }
    }

    pub fn has_data_to_read_from_query_or_path_or_header(&self) -> bool {
        for field in &self.fields {
            match &field.src {
                InputFieldSource::Query => return true,
                InputFieldSource::Path => return true,
                InputFieldSource::Header => return true,
                InputFieldSource::Body => {}
                InputFieldSource::FormData => {}
                InputFieldSource::BodyFile => {}
            }
        }
        return false;
    }

    pub fn has_body_data_to_read(&self) -> Option<BodyDataToReader> {
        {
            let mut body_attrs_amount = 0;
            let mut last_body_type = None;

            for field in &self.fields {
                match &field.src {
                    InputFieldSource::Query => {}
                    InputFieldSource::Path => {}
                    InputFieldSource::Header => {}
                    InputFieldSource::Body => {
                        body_attrs_amount += 1;
                        last_body_type = Some(&field.property.ty);
                    }
                    InputFieldSource::FormData => {
                        body_attrs_amount += 1;
                    }
                    InputFieldSource::BodyFile => {
                        body_attrs_amount += 1;
                    }
                }
            }
            if let Some(last_body_type) = last_body_type {
                if body_attrs_amount == 1 {
                    if last_body_type.is_vec_of_u8() {
                        return Some(BodyDataToReader::RawBodyToVec);
                    } else {
                        return Some(BodyDataToReader::DeserializeBody);
                    }
                }
            }
        }

        for field in &self.fields {
            match &field.src {
                InputFieldSource::Query => {}
                InputFieldSource::Path => {}
                InputFieldSource::Header => {}
                InputFieldSource::Body => {
                    if field.property.ty.is_vec_of_u8() {
                        return Some(BodyDataToReader::RawBodyToVec);
                    }

                    return Some(BodyDataToReader::BodyModel);
                }
                InputFieldSource::FormData => {
                    return Some(BodyDataToReader::FormData);
                }
                InputFieldSource::BodyFile => {
                    return Some(BodyDataToReader::BodyFile);
                }
            }
        }
        return None;
    }
}

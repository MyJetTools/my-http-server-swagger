use macros_utils::{AttributeParams, ParamValue};

use crate::reflection::StructProperty;

pub enum BodyDataToReader {
    FormData,
    BodyFile,
    RawBodyToVec,
    DeserializeBody,
    BodyModel,
}

#[derive(Debug)]
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
    pub my_attr: AttributeParams,
}

fn get_attr(property: &StructProperty) -> Option<(String, InputFieldSource)> {
    for name in property.attrs.keys() {
        let src = InputFieldSource::from_str(name);

        if let Some(src) = src {
            return Some((name.to_string(), src));
        }
    }
    None
}

impl InputField {
    pub fn new(mut property: StructProperty) -> Option<Self> {
        let (attr_name, src) = get_attr(&property)?;

        let attr = property.attrs.remove(&attr_name).unwrap();

        if attr.is_none() {
            panic!("Attribute {} does not have any description", attr_name);
        }

        return Self {
            property,
            src,
            my_attr: attr.unwrap(),
        }
        .into();
    }

    pub fn name(&self) -> String {
        if let Some(value) = self.my_attr.get_named_param("name") {
            value.get_value_as_str().to_string()
        } else {
            self.property.name.clone()
        }
    }

    pub fn required(&self) -> bool {
        if self.property.ty.is_vec() {
            return false;
        }

        !self.property.ty.is_option()
    }

    pub fn get_default_value(&self) -> Option<ParamValue> {
        self.my_attr.get_named_param("default")
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

    pub fn get_body_type(&self) -> Option<ParamValue> {
        self.my_attr.get_named_param("body_type")
    }

    pub fn description(&self) -> ParamValue {
        if let Some(value) = self.my_attr.get_named_param("description") {
            return value;
        }

        panic!(
            "Description field is missing of the field {}",
            self.property.name
        );
    }

    pub fn validator(&self) -> Option<ParamValue> {
        let result = self.my_attr.get_named_param("validator")?;
        Some(result)
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
                        last_body_type = Some(field);
                    }
                    InputFieldSource::FormData => {
                        body_attrs_amount += 1;
                    }
                    InputFieldSource::BodyFile => {
                        body_attrs_amount += 1;
                    }
                }
            }

            if let Some(last_input_field) = last_body_type {
                if body_attrs_amount == 1 {
                    match &last_input_field.property.ty {
                        crate::reflection::PropertyType::U8 => {}
                        crate::reflection::PropertyType::I8 => {}
                        crate::reflection::PropertyType::U16 => {}
                        crate::reflection::PropertyType::I16 => {}
                        crate::reflection::PropertyType::U32 => {}
                        crate::reflection::PropertyType::I32 => {}
                        crate::reflection::PropertyType::U64 => {}
                        crate::reflection::PropertyType::I64 => {}
                        crate::reflection::PropertyType::F32 => {}
                        crate::reflection::PropertyType::F64 => {}
                        crate::reflection::PropertyType::USize => {}
                        crate::reflection::PropertyType::ISize => {}
                        crate::reflection::PropertyType::String => {}
                        crate::reflection::PropertyType::Str => {}
                        crate::reflection::PropertyType::Bool => {}
                        crate::reflection::PropertyType::DateTime => {}
                        crate::reflection::PropertyType::FileContent => {}
                        crate::reflection::PropertyType::OptionOf(_) => {}
                        crate::reflection::PropertyType::VecOf(sub_type) => {
                            if sub_type.is_u8() {
                                return Some(BodyDataToReader::RawBodyToVec);
                            }
                            return Some(BodyDataToReader::DeserializeBody);
                        }
                        crate::reflection::PropertyType::Struct(_) => {
                            return Some(BodyDataToReader::DeserializeBody)
                        }
                    };
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

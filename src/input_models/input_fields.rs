use macros_utils::{AttributeParams, ParamValue};
use proc_macro2::TokenStream;
use types_reader::{PropertyType, StructProperty};

use crate::proprety_type_ext::PropertyTypeExt;

pub struct BodyNotBodyFields<'s> {
    pub body_fields: Option<Vec<&'s InputField<'s>>>,
    pub not_body_fields: Option<Vec<&'s InputField<'s>>>,
}

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

pub struct InputField<'s> {
    pub property: StructProperty<'s>,
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

impl<'s> InputField<'s> {
    pub fn new(mut property: StructProperty<'s>) -> Option<Self> {
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

    pub fn name(&self) -> ParamValue {
        if let Some(value) = self.my_attr.get_named_param("name") {
            value
        } else {
            ParamValue {
                value: self.property.name.as_bytes(),
            }
        }
    }

    pub fn required(&self) -> bool {
        match &self.property.ty {
            PropertyType::VecOf(_) => return false,
            _ => {}
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

    pub fn get_body_type(&self) -> Option<ParamValue> {
        self.my_attr.get_named_param("body_type")
    }

    pub fn description(&self) -> Result<ParamValue, syn::Error> {
        if let Some(value) = self.my_attr.get_named_param("description") {
            return Ok(value);
        }

        let err =
            syn::Error::new_spanned(self.property.field, "description is missing of the field");

        Err(err)
    }

    pub fn validator(&self) -> Option<ParamValue> {
        let result = self.my_attr.get_named_param("validator")?;
        Some(result)
    }

    pub fn get_struct_fiel_name_as_token_stream(&self) -> TokenStream {
        let name = self.property.get_field_name_ident();
        quote::quote!(#name)
    }
}

pub struct InputFields<'s> {
    pub fields: Vec<InputField<'s>>,
}

impl<'s> InputFields<'s> {
    pub fn new(src: Vec<StructProperty<'s>>) -> Self {
        let mut fields = Vec::new();

        for prop in src {
            if let Some(field) = InputField::new(prop) {
                fields.push(field);
            }
        }

        Self { fields }
    }

    pub fn get_body_and_not_body_fields(&'s self) -> BodyNotBodyFields<'s> {
        self.check_types_of_field();
        let mut body_fields = rust_extensions::lazy::LazyVec::with_capacity(self.fields.len());
        let mut not_body_fields = rust_extensions::lazy::LazyVec::with_capacity(self.fields.len());

        for field in &self.fields {
            if field.is_reading_from_body() {
                body_fields.add(field);
            } else {
                not_body_fields.add(field);
            }
        }

        BodyNotBodyFields {
            body_fields: body_fields.get_result(),
            not_body_fields: not_body_fields.get_result(),
        }
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
                        types_reader::PropertyType::U8 => {}
                        types_reader::PropertyType::I8 => {}
                        types_reader::PropertyType::U16 => {}
                        types_reader::PropertyType::I16 => {}
                        types_reader::PropertyType::U32 => {}
                        types_reader::PropertyType::I32 => {}
                        types_reader::PropertyType::U64 => {}
                        types_reader::PropertyType::I64 => {}
                        types_reader::PropertyType::F32 => {}
                        types_reader::PropertyType::F64 => {}
                        types_reader::PropertyType::USize => {}
                        types_reader::PropertyType::ISize => {}
                        types_reader::PropertyType::String => {}
                        types_reader::PropertyType::Str => {}
                        types_reader::PropertyType::Bool => {}
                        types_reader::PropertyType::DateTime => {}
                        types_reader::PropertyType::OptionOf(_) => {}
                        types_reader::PropertyType::VecOf(sub_type) => {
                            if sub_type.is_u8() {
                                return Some(BodyDataToReader::RawBodyToVec);
                            }
                            return Some(BodyDataToReader::DeserializeBody);
                        }
                        types_reader::PropertyType::Struct(..) => {
                            if !last_input_field.property.ty.is_file_content() {
                                return Some(BodyDataToReader::DeserializeBody);
                            }
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

use macros_utils::{AttributeParams, ParamValue};
use proc_macro2::TokenStream;
use types_reader::{PropertyType, StructProperty};

use crate::proprety_type_ext::PropertyTypeExt;

pub struct BodyNotBodyFields<'s> {
    pub body_fields: Option<Vec<&'s InputField<'s>>>,
    pub not_body_fields: Option<Vec<&'s InputField<'s>>>,
}

pub struct BodyDataToReader {
    pub body_file: usize,
    pub body_model: usize,
    pub body_raw: usize,
    pub body_field: usize,
    pub form_data_field: usize,
}

impl BodyDataToReader {
    pub fn has_form_data(&self) -> bool {
        self.form_data_field > 0
    }

    pub fn has_body_data(&self) -> bool {
        self.body_file > 0 || self.body_model > 0 || self.body_raw > 0 || self.body_field > 0
    }

    pub fn is_empty(&self) -> bool {
        self.body_file > 0
            && self.body_field == 0
            && self.body_model == 0
            && self.form_data_field == 0
    }
}

#[derive(Debug)]
pub enum InputFieldSource {
    Query,
    Path,
    Header,
    Body,
    BodyFile,
    FormData,
}

impl InputFieldSource {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            "http_query" => Some(Self::Query),
            "http_header" => Some(Self::Header),
            "http_path" => Some(Self::Path),
            "http_form_data" => Some(Self::FormData),
            "http_body" => Some(Self::Body),
            "http_body_file" => Some(Self::Body),
            _ => None,
        }
    }

    pub fn is_body(&self) -> bool {
        match self {
            Self::Body => true,
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

    pub fn has_body_data_to_read(&self) -> Result<Option<BodyDataToReader>, syn::Error> {
        let mut body_data_reader = BodyDataToReader {
            body_file: 0,
            form_data_field: 0,
            body_field: 0,
            body_model: 0,
            body_raw: 0,
        };

        for field in &self.fields {
            match &field.src {
                InputFieldSource::Body => {
                    if body_data_reader.has_form_data() {
                        let err = syn::Error::new_spanned(
                            field.property.field,
                            "Form data and body data can not be mixed",
                        );
                        return Err(err);
                    };

                    if field.property.ty.is_file_content() {
                        if body_data_reader.body_raw > 1 {
                            let err = syn::Error::new_spanned(
                                field.property.field,
                                "Field is already attributed as reading raw body to vec of u8",
                            );
                            return Err(err);
                        }

                        if body_data_reader.body_field > 1 {
                            let err = syn::Error::new_spanned(
                                field.property.field,
                                "Field is already attributed as reading body model",
                            );
                            return Err(err);
                        }

                        body_data_reader.body_file += 1;
                        if body_data_reader.body_file > 1 {
                            let err = syn::Error::new_spanned(
                                field.property.field,
                                "Only one field can be attributed as body_file",
                            );
                            return Err(err);
                        }
                    } else if field.property.ty.is_vec_of_u8() {
                        if body_data_reader.body_file > 1 {
                            let err = syn::Error::new_spanned(
                                field.property.field,
                                "Field is already attributed as reading body file",
                            );
                            return Err(err);
                        }

                        if body_data_reader.body_field > 1 {
                            let err = syn::Error::new_spanned(
                                field.property.field,
                                "Field is already attributed as reading body model",
                            );
                            return Err(err);
                        }

                        body_data_reader.body_raw += 1;

                        if body_data_reader.body_raw > 1 {
                            let err = syn::Error::new_spanned(
                                field.property.field,
                                "Only one field can be complietly serrialized from body",
                            );
                            return Err(err);
                        }
                    } else if field.property.ty.is_struct() {
                        body_data_reader.body_model += 1;

                        if body_data_reader.body_model > 1 {
                            let err = syn::Error::new_spanned(
                                field.property.field,
                                "Only one field can be complietly serrialized from body",
                            );
                            return Err(err);
                        }
                    } else {
                        body_data_reader.body_field += 1;
                    }
                }
                InputFieldSource::FormData => {
                    if body_data_reader.has_body_data() {
                        let err = syn::Error::new_spanned(
                            field.property.field,
                            "Form data and body data can not be mixed",
                        );
                        return Err(err);
                    };

                    body_data_reader.form_data_field += 1;
                }
                _ => {}
            }
        }

        if body_data_reader.is_empty() {
            Ok(None)
        } else {
            Ok(Some(body_data_reader))
        }
    }
}

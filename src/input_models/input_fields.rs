use proc_macro2::TokenStream;
use rust_extensions::lazy::LazyVec;
use types_reader::{
    attribute_params::{AttributeParams, ParamValue},
    StructProperty,
};

pub struct BodyNotBodyFields<'s> {
    pub body_fields: Option<Vec<&'s InputField<'s>>>,
    pub not_body_fields: Option<Vec<&'s InputField<'s>>>,
    pub path_fields: Option<Vec<&'s InputField<'s>>>,
}

pub struct BodyDataToReader {
    pub http_form: usize,
    pub http_body: usize,
    pub http_body_raw: usize,
}

impl BodyDataToReader {
    pub fn has_form_data(&self) -> bool {
        self.http_form > 0
    }

    pub fn has_body_data(&self) -> bool {
        self.http_body > 0
    }

    pub fn has_body_raw_data(&self) -> bool {
        self.http_body_raw > 0
    }

    pub fn is_empty(&self) -> bool {
        self.http_form == 0 && self.http_body == 0
    }
}

#[derive(Debug)]
pub enum InputFieldSource {
    Query,
    Path,
    Header,
    Body,
    FormData,
    BodyRaw,
}

impl InputFieldSource {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            "http_query" => Some(Self::Query),
            "http_header" => Some(Self::Header),
            "http_path" => Some(Self::Path),
            "http_form_data" => Some(Self::FormData),
            "http_body" => Some(Self::Body),
            "http_body_raw" => Some(Self::BodyRaw),
            _ => None,
        }
    }

    pub fn is_path(&self) -> bool {
        matches!(self, Self::Path)
    }
}

pub struct InputField<'s> {
    pub property: StructProperty<'s>,
    pub src: InputFieldSource,
    attr_name: String,
}

fn get_attr(property: &StructProperty) -> Option<(String, InputFieldSource)> {
    for name in property.attrs.get_attr_names() {
        let src = InputFieldSource::from_str(name);

        if let Some(src) = src {
            return Some((name.to_string(), src));
        }
    }
    None
}

impl<'s> InputField<'s> {
    pub fn new(property: StructProperty<'s>) -> Result<Option<Self>, syn::Error> {
        let get_attr_result = get_attr(&property);

        if get_attr_result.is_none() {
            return Ok(None);
        }

        let (attr_name, src) = get_attr_result.unwrap();

        return Ok(Self {
            property,
            src,
            attr_name,
        }
        .into());
    }

    fn get_my_attr(&'s self) -> &AttributeParams<'s> {
        self.property.attrs.get_attr(&self.attr_name).unwrap()
    }

    pub fn name(&self) -> ParamValue {
        if let Ok(value) = self.get_my_attr().get_named_param("name") {
            value
        } else {
            ParamValue {
                value: self.property.name.as_bytes(),
            }
        }
    }

    pub fn get_default_value(&self) -> Option<ParamValue> {
        self.get_my_attr().get_named_param("default").ok()
    }

    pub fn is_reading_from_body(&self) -> bool {
        match self.src {
            InputFieldSource::Query => false,
            InputFieldSource::Path => false,
            InputFieldSource::Header => false,
            InputFieldSource::Body => true,
            InputFieldSource::FormData => true,
            InputFieldSource::BodyRaw => true,
        }
    }

    pub fn description(&self) -> Result<ParamValue, syn::Error> {
        self.get_my_attr().get_named_param("description")
    }

    pub fn validator(&self) -> Option<ParamValue> {
        self.get_my_attr().get_named_param("validator").ok()
    }

    pub fn get_struct_field_name_as_token_stream(&self) -> TokenStream {
        let name = self.property.get_field_name_ident();
        quote::quote!(#name)
    }
}

pub struct InputFields<'s> {
    pub fields: Vec<InputField<'s>>,
}

impl<'s> InputFields<'s> {
    pub fn new(src: Vec<StructProperty<'s>>) -> Result<Self, syn::Error> {
        let mut fields = Vec::new();

        for prop in src {
            if let Some(field) = InputField::new(prop)? {
                fields.push(field);
            }
        }

        Ok(Self { fields })
    }

    pub fn get_body_and_not_body_fields(&'s self) -> BodyNotBodyFields<'s> {
        let mut body_fields = rust_extensions::lazy::LazyVec::with_capacity(self.fields.len());
        let mut not_body_fields = rust_extensions::lazy::LazyVec::with_capacity(self.fields.len());

        let mut path_fields = rust_extensions::lazy::LazyVec::with_capacity(self.fields.len());

        for field in &self.fields {
            if field.is_reading_from_body() {
                body_fields.add(field);
            }
            if field.src.is_path() {
                path_fields.add(field);
            } else {
                not_body_fields.add(field);
            }
        }

        BodyNotBodyFields {
            body_fields: body_fields.get_result(),
            not_body_fields: not_body_fields.get_result(),
            path_fields: path_fields.get_result(),
        }
    }

    pub fn has_body_data_to_read(&self) -> Result<Option<BodyDataToReader>, syn::Error> {
        let mut body_data_reader = BodyDataToReader {
            http_form: 0,
            http_body: 0,
            http_body_raw: 0,
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

                    if body_data_reader.has_body_raw_data() {
                        let err = syn::Error::new_spanned(
                            field.property.field,
                            "Body data and 'body raw' data can not be mixed",
                        );
                        return Err(err);
                    };

                    body_data_reader.http_body += 1;
                }
                InputFieldSource::FormData => {
                    if body_data_reader.has_body_data() {
                        let err = syn::Error::new_spanned(
                            field.property.field,
                            "Form data and body data can not be mixed",
                        );
                        return Err(err);
                    };

                    if body_data_reader.has_body_raw_data() {
                        let err = syn::Error::new_spanned(
                            field.property.field,
                            "Form data and 'body raw' data can not be mixed",
                        );
                        return Err(err);
                    };

                    body_data_reader.http_form += 1;
                }

                InputFieldSource::BodyRaw => {
                    if body_data_reader.has_form_data() {
                        let err = syn::Error::new_spanned(
                            field.property.field,
                            "Form data and body data can not be mixed",
                        );
                        return Err(err);
                    };

                    if body_data_reader.has_body_data() {
                        let err = syn::Error::new_spanned(
                            field.property.field,
                            "Body data and 'body raw' data can not be mixed",
                        );
                        return Err(err);
                    };

                    body_data_reader.http_body_raw += 1;

                    if body_data_reader.http_body_raw > 1 {
                        let err = syn::Error::new_spanned(
                            field.property.field,
                            "Body raw data can be only once",
                        );
                        return Err(err);
                    };
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

    pub fn get_routes(&self) -> Option<Vec<&InputField>> {
        let mut result = LazyVec::new();

        for field in &self.fields {
            if field.src.is_path() {
                result.add(field);
            }
        }

        result.get_result()
    }
}

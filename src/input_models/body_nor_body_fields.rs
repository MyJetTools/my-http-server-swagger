use types_reader::StructProperty;

use super::{input_model_struct_property_ext::InputModelStructPropertyExt, InputField};

pub struct BodyNotBodyFields<'s> {
    pub body_fields: Option<Vec<&'s StructProperty<'s>>>,
    pub not_body_fields: Option<Vec<&'s StructProperty<'s>>>,
    pub path_fields: Option<Vec<&'s StructProperty<'s>>>,
}

impl<'s> BodyNotBodyFields<'s> {
    pub fn new(props: &'s [StructProperty]) -> Result<Self, syn::Error> {
        let mut body_fields = rust_extensions::lazy::LazyVec::with_capacity(props.len());
        let mut not_body_fields = rust_extensions::lazy::LazyVec::with_capacity(props.len());

        let mut path_fields = rust_extensions::lazy::LazyVec::with_capacity(props.len());

        for struct_property in props {
            let http_input = struct_property.get_input_field()?;
            if http_input.is_body() {
                body_fields.add(struct_property);
            } else if http_input.is_path() {
                path_fields.add(struct_property);
            } else {
                not_body_fields.add(struct_property);
            }
        }

        Ok(Self {
            body_fields: body_fields.get_result(),
            not_body_fields: not_body_fields.get_result(),
            path_fields: path_fields.get_result(),
        })
    }

    pub fn has_body_data_to_read(&self) -> Result<Option<BodyDataToReader>, syn::Error> {
        if self.body_fields.is_none() {
            return Ok(None);
        }

        let body_fields = self.body_fields.as_ref().unwrap();

        let mut body_data_reader = BodyDataToReader {
            http_form: 0,
            http_body: 0,
            http_body_raw: 0,
        };

        for field in body_fields {
            let input_data = field.get_input_field()?;

            match &input_data {
                InputField::Body(field_data) => {
                    if body_data_reader.has_form_data() {
                        let err = syn::Error::new_spanned(
                            field_data.property.field,
                            "Form data and body data can not be mixed",
                        );
                        return Err(err);
                    };

                    if body_data_reader.has_body_raw_data() {
                        let err = syn::Error::new_spanned(
                            field_data.property.field,
                            "Body data and 'body raw' data can not be mixed",
                        );
                        return Err(err);
                    };

                    body_data_reader.http_body += 1;
                }
                InputField::FormData(field_data) => {
                    if body_data_reader.has_body_data() {
                        let err = syn::Error::new_spanned(
                            field_data.property.field,
                            "Form data and body data can not be mixed",
                        );
                        return Err(err);
                    };

                    if body_data_reader.has_body_raw_data() {
                        let err = syn::Error::new_spanned(
                            field_data.property.field,
                            "Form data and 'body raw' data can not be mixed",
                        );
                        return Err(err);
                    };

                    body_data_reader.http_form += 1;
                }

                InputField::BodyRaw(field_data) => {
                    if body_data_reader.has_form_data() {
                        let err = syn::Error::new_spanned(
                            field_data.property.field,
                            "Form data and body data can not be mixed",
                        );
                        return Err(err);
                    };

                    if body_data_reader.has_body_data() {
                        let err = syn::Error::new_spanned(
                            field_data.property.field,
                            "Body data and 'body raw' data can not be mixed",
                        );
                        return Err(err);
                    };

                    body_data_reader.http_body_raw += 1;

                    if body_data_reader.http_body_raw > 1 {
                        let err = syn::Error::new_spanned(
                            field_data.property.field,
                            "Body raw data can be only once",
                        );
                        return Err(err);
                    };
                }

                _ => {}
            }
        }

        Ok(Some(body_data_reader))
    }
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
}

/*
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

    fn get_my_attr(&'s self) -> &AttributeParams {
        self.property.attrs.get_attr(&self.attr_name).unwrap()
    }

    pub fn name(&self) -> ParamValue {
        if let Ok(value) = self.get_my_attr().get_named_param("name") {
            value
        } else {
            ParamValue {
                value: self.property.name.as_bytes(),
                token: None,
                ident: Some(self.property.get_field_name_ident()),
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
            } else if field.src.is_path() {
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
 */

use rust_extensions::lazy::LazyVec;
use types_reader::StructProperty;

use super::{input_model_struct_property_ext::InputModelStructPropertyExt, InputField};

pub struct BodyNotBodyFields<'s> {
    pub header_fields: Option<Vec<&'s StructProperty<'s>>>,
    pub query_string_fields: Option<Vec<&'s StructProperty<'s>>>,
    pub body_fields: Option<Vec<&'s StructProperty<'s>>>,
    pub path_fields: Option<Vec<&'s StructProperty<'s>>>,
}

impl<'s> BodyNotBodyFields<'s> {
    pub fn new(props: &'s [StructProperty]) -> Result<Self, syn::Error> {
        let mut body_fields = LazyVec::with_capacity(props.len());
        let mut query_string_fields = LazyVec::with_capacity(props.len());
        let mut header_fields = LazyVec::with_capacity(props.len());

        let mut path_fields = LazyVec::with_capacity(props.len());

        for struct_property in props {
            let http_input = struct_property.get_input_field()?;
            if http_input.is_body() {
                body_fields.add(struct_property);
            } else if http_input.is_path() {
                path_fields.add(struct_property);
            }
            if http_input.is_header() {
                header_fields.add(struct_property);
            } else {
                query_string_fields.add(struct_property);
            }
        }

        Ok(Self {
            body_fields: body_fields.get_result(),
            header_fields: header_fields.get_result(),
            query_string_fields: query_string_fields.get_result(),
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

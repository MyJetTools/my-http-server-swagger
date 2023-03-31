use types_reader::StructProperty;

use super::{InputField, InputFieldData};

const HTTP_PATH_NAME: &'static str = "http_path";

pub trait InputModelStructPropertyExt {
    fn get_input_field(&self) -> Result<InputField, syn::Error>;
    fn try_into_input_path_field(&self) -> Result<Option<InputField>, syn::Error>;
    fn get_struct_field_name_as_token_stream(&self) -> proc_macro2::TokenStream;
}

impl<'s> InputModelStructPropertyExt for StructProperty<'s> {
    fn get_input_field(&self) -> Result<InputField, syn::Error> {
        for attr_name in self.attrs.get_attr_names() {
            let attr = self.attrs.get_attr(attr_name)?;
            match attr_name.as_str() {
                "http_query" => return Ok(InputField::Query(InputFieldData::new(&self, attr))),
                "http_header" => return Ok(InputField::Header(InputFieldData::new(&self, attr))),
                HTTP_PATH_NAME => return Ok(InputField::Path(InputFieldData::new(&self, attr))),
                "http_form_data" => {
                    return Ok(InputField::FormData(InputFieldData::new(&self, attr)))
                }
                "http_body" => return Ok(InputField::Body(InputFieldData::new(&self, attr))),
                "http_body_raw" => {
                    return Ok(InputField::BodyRaw(InputFieldData::new(&self, attr)))
                }
                _ => {}
            }
        }
        return {
            let attrs: Vec<&String> = self.attrs.get_attr_names().collect();
            Err(syn::Error::new(
            self.get_field_name_ident().span(),
            format!("Please specify http_query, http_header, http_path, http_form_data or http_body for {}. Found attrs: {:?}", self.name, attrs),
        ))
        };
    }

    fn try_into_input_path_field(&self) -> Result<Option<InputField>, syn::Error> {
        for attr_name in self.attrs.get_attr_names() {
            let attr = self.attrs.get_attr(attr_name)?;
            match attr_name.as_str() {
                HTTP_PATH_NAME => {
                    return Ok(Some(InputField::Path(InputFieldData::new(&self, attr))))
                }
                _ => {}
            }
        }
        Ok(None)
    }

    fn get_struct_field_name_as_token_stream(&self) -> proc_macro2::TokenStream {
        let name = self.get_field_name_ident();
        quote::quote!(#name)
    }
}

use proc_macro2::TokenStream;
use types_reader::{ParamsList, StructProperty};

pub struct InputFieldData<'s> {
    pub property: &'s StructProperty<'s>,
    pub attr_params: &'s ParamsList,
}

impl<'s> InputFieldData<'s> {
    pub fn new(property: &'s StructProperty<'s>, attr_params: &'s ParamsList) -> Self {
        Self {
            property,
            attr_params,
        }
    }

    pub fn get_input_field_name(&self) -> Result<&str, syn::Error> {
        if let Some(value) = self.attr_params.try_get_named_param("name") {
            Ok(value.get_str_value()?)
        } else {
            Ok(&self.property.name)
        }
    }

    pub fn get_default_value(&self) -> Result<Option<&str>, syn::Error> {
        match self.attr_params.try_get_named_param("default") {
            Some(value) => Ok(Some(value.get_str_value()?)),
            None => Ok(None),
        }
    }

    pub fn get_description(&self) -> Result<&str, syn::Error> {
        let result = self.attr_params.get_named_param("description")?;
        result.get_str_value()
    }

    pub fn validator(&self) -> Result<Option<&str>, syn::Error> {
        let result = self.attr_params.try_get_named_param("validator");

        match result {
            Some(value) => Ok(Some(value.get_str_value()?)),
            _ => Ok(None),
        }
    }
}

pub enum InputField<'s> {
    Query(InputFieldData<'s>),
    Path(InputFieldData<'s>),
    Header(InputFieldData<'s>),
    Body(InputFieldData<'s>),
    FormData(InputFieldData<'s>),
    BodyRaw(InputFieldData<'s>),
}

impl<'s> InputField<'s> {
    /*
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
    */

    pub fn get_input_data(&'s self) -> &'s InputFieldData<'s> {
        match self {
            Self::Query(data) => data,
            Self::Path(data) => data,
            Self::Header(data) => data,
            Self::Body(data) => data,
            Self::FormData(data) => data,
            Self::BodyRaw(data) => data,
        }
    }
    pub fn is_path(&self) -> bool {
        match self {
            Self::Path(_) => true,
            _ => false,
        }
    }

    pub fn get_input_src_token(&self) -> TokenStream {
        let http_parameter_input_src = crate::consts::get_http_parameter_input_src();
        match self {
            Self::Query(_) => quote::quote!(#http_parameter_input_src::Query),
            Self::Path(_) => quote::quote!(#http_parameter_input_src::Path),
            Self::Header(_) => quote::quote!(#http_parameter_input_src::Header),
            Self::Body(_) => quote::quote!(#http_parameter_input_src::BodyModel),
            Self::FormData(_) => quote::quote!(#http_parameter_input_src::FormData),
            Self::BodyRaw(_) => quote:: quote!(#http_parameter_input_src::BodyRaw),
        }
    }

    pub fn is_body(&self) -> bool {
        match self {
            Self::Body(_) => true,
            Self::BodyRaw(_) => true,
            Self::FormData(_) => true,
            _ => false,
        }
    }

    pub fn get_input_field_name(&self) -> Result<&str, syn::Error> {
        let data = self.get_input_data();
        data.get_input_field_name()
    }

    pub fn get_default_value(&self) -> Result<Option<&str>, syn::Error> {
        let data = self.get_input_data();
        data.get_default_value()
    }

    pub fn validator(&self) -> Result<Option<&str>, syn::Error> {
        let data = self.get_input_data();
        data.validator()
    }
}

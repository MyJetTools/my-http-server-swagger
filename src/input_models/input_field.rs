use types_reader::{ParamValue, ParamsList, StructProperty};

#[derive(Clone)]
pub enum HttpInputSource {
    Query,
    Header,
    FormData,
    Body,
    BodyRaw,
    Path,
}

impl HttpInputSource {
    pub fn get_input_src_token(&self) -> proc_macro2::TokenStream {
        let http_parameter_input_src = crate::consts::get_http_parameter_input_src();
        match self {
            Self::Query => quote::quote!(#http_parameter_input_src::Query),
            Self::Path => quote::quote!(#http_parameter_input_src::Path),
            Self::Header => quote::quote!(#http_parameter_input_src::Header),
            Self::Body => quote::quote!(#http_parameter_input_src::BodyModel),
            Self::FormData => quote::quote!(#http_parameter_input_src::FormData),
            Self::BodyRaw => quote:: quote!(#http_parameter_input_src::BodyRaw),
        }
    }
}

pub enum DefaultValue<'s> {
    Empty(&'s ParamValue),
    Value(&'s str),
}

impl<'s> DefaultValue<'s> {
    pub fn unwrap_value(&'s self) -> Result<&'s str, syn::Error> {
        match self {
            DefaultValue::Empty(value) => Err(value.throw_error("Default value is not specified")),
            DefaultValue::Value(value) => Ok(value),
        }
    }
}

#[derive(Clone)]
pub struct InputField<'s> {
    pub property: &'s StructProperty<'s>,
    pub attr_params: &'s ParamsList,
    pub src: HttpInputSource,
}

impl<'s> InputField<'s> {
    pub fn new(
        property: &'s StructProperty<'s>,
        attr_params: &'s ParamsList,
        src: HttpInputSource,
    ) -> Self {
        Self {
            property,
            attr_params,
            src,
        }
    }

    pub fn get_input_field_name(&self) -> Result<&str, syn::Error> {
        if let Some(value) = self.attr_params.try_get_named_param("name") {
            Ok(value.unwrap_as_string_value()?.into())
        } else {
            Ok(&self.property.name)
        }
    }

    pub fn get_default_value(&self) -> Result<Option<DefaultValue>, syn::Error> {
        match self.attr_params.try_get_named_param("default") {
            Some(value) => {
                if value.is_none().is_some() {
                    return Ok(Some(DefaultValue::Empty(value)));
                }

                let value = value.get_any_value_as_str()?.into();

                Ok(Some(DefaultValue::Value(value)))
            }
            None => Ok(None),
        }
    }

    pub fn get_description(&self) -> Result<&str, syn::Error> {
        let result = self.attr_params.get_named_param("description")?;
        Ok(result.unwrap_as_string_value()?.into())
    }

    pub fn validator(&self) -> Result<Option<&str>, syn::Error> {
        let result = self.attr_params.try_get_named_param("validator");

        match result {
            Some(value) => Ok(Some(value.unwrap_as_string_value()?.into())),
            _ => Ok(None),
        }
    }
}

/*
impl<'s> InputField<'s> {
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



    pub fn is_body(&self) -> bool {
        match self {
            Self::Body(_) => true,
            Self::BodyRaw(_) => true,
            Self::FormData(_) => true,
            _ => false,
        }
    }

    pub fn is_body_raw(&self) -> bool {
        match self {
            Self::Body(_) => true,
            Self::BodyRaw(_) => true,
            Self::FormData(_) => true,
            _ => false,
        }
    }

    pub fn is_header(&self) -> bool {
        match self {
            Self::Header(_) => true,
            _ => false,
        }
    }

    pub fn get_input_field_name(&self) -> Result<&str, syn::Error> {
        let data = self.get_input_data();
        data.get_input_field_name()
    }

    pub fn get_default_value(&self) -> Result<Option<DefaultValue>, syn::Error> {
        let data = self.get_input_data();
        data.get_default_value()
    }

    pub fn validator(&self) -> Result<Option<&str>, syn::Error> {
        let data = self.get_input_data();
        data.validator()
    }
}
 */

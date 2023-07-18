use proc_macro2::TokenStream;
use types_reader::{ParamValue, ParamsList, PropertyType, StructProperty};

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
    Value(&'s ParamValue),
}

impl<'s> DefaultValue<'s> {
    pub fn unwrap_value(&'s self) -> Result<&'s ParamValue, syn::Error> {
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

                Ok(Some(DefaultValue::Value(value)))
            }
            None => Ok(None),
        }
    }

    pub fn get_default_value_opt_case(&self) -> Result<TokenStream, syn::Error> {
        if let Some(default) = self.get_default_value()? {
            let value = default.unwrap_value()?;
            if let PropertyType::OptionOf(pt) = &self.property.ty {
                match pt.as_ref() {
                    PropertyType::U8 => {
                        let value = value.unwrap_as_number_value()?.as_u8();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::I8 => {
                        let value = value.unwrap_as_number_value()?.as_i8();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::U16 => {
                        let value = value.unwrap_as_number_value()?.as_u16();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::I16 => {
                        let value = value.unwrap_as_number_value()?.as_i16();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::U32 => {
                        let value = value.unwrap_as_number_value()?.as_u32();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::I32 => {
                        let value = value.unwrap_as_number_value()?.as_i32();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::U64 => {
                        let value = value.unwrap_as_number_value()?.as_u64();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::I64 => {
                        let value = value.unwrap_as_number_value()?.as_i64();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::F32 => {
                        let value = value.unwrap_as_double_value()?.as_f32();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::F64 => {
                        let value = value.unwrap_as_double_value()?.as_f64();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::USize => {
                        let value = value.unwrap_as_number_value()?.as_i64() as usize;
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::ISize => {
                        let value = value.unwrap_as_number_value()?.as_i64() as isize;
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::String => {
                        let value = value.unwrap_as_string_value()?.as_str();
                        return Ok(quote::quote!(Some(#value.to_string())));
                    }
                    PropertyType::Str => {
                        let value = value.unwrap_as_string_value()?.as_str();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::Bool => {
                        let value = value.unwrap_as_bool_value()?.get_value();
                        return Ok(quote::quote!(Some(#value)));
                    }
                    PropertyType::DateTime => {
                        let value = value.get_any_value_as_str()?;
                        return Ok(quote::quote!(DateTimeAsMicroseconds::from_str(#value)));
                    }
                    PropertyType::OptionOf(_) => {
                        return Ok(quote::quote!(None));
                    }
                    PropertyType::VecOf(_) => {
                        return Ok(quote::quote!(None));
                    }
                    PropertyType::Struct(_, _) => {
                        return Ok(quote::quote!(None));
                    }
                    PropertyType::HashMap(_, _) => {
                        return Ok(quote::quote!(None));
                    }
                }
            }
        }

        Ok(quote::quote!(None))
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

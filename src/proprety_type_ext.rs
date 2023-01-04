use proc_macro2::TokenStream;
use types_reader::PropertyType;

pub trait PropertyTypeExt {
    fn get_swagger_simple_type(&self, is_password: bool) -> Option<TokenStream>;

    fn is_file_content(&self) -> bool;

    fn is_raw_body(&self) -> bool;
}

impl<'s> PropertyTypeExt for PropertyType<'s> {
    fn is_file_content(&self) -> bool {
        match self {
            PropertyType::Struct(name, _) => {
                println!("name: {}", name);
                name == "FileContent"
            }
            _ => false,
        }
    }

    fn is_raw_body(&self) -> bool {
        match self {
            PropertyType::Struct(name, _) => {
                println!("name: {}", name);
                name == "RawData"
            }
            _ => false,
        }
    }

    fn get_swagger_simple_type(&self, is_password: bool) -> Option<TokenStream> {
        use quote::quote;
        let http_simple_type = crate::consts::get_http_simple_type();

        match self {
            PropertyType::String => {
                if is_password {
                    quote!(#http_simple_type::Password).into()
                } else {
                    quote!(#http_simple_type::String).into()
                }
            }
            PropertyType::Str => {
                if is_password {
                    quote!(#http_simple_type::Password).into()
                } else {
                    quote!(#http_simple_type::String).into()
                }
            }
            PropertyType::U8 => quote!(#http_simple_type::Integer).into(),
            PropertyType::I8 => quote!(#http_simple_type::Integer).into(),
            PropertyType::U16 => quote!(#http_simple_type::Integer).into(),
            PropertyType::I16 => quote!(#http_simple_type::Integer).into(),
            PropertyType::U32 => quote!(#http_simple_type::Integer).into(),
            PropertyType::I32 => quote!(#http_simple_type::Integer).into(),
            PropertyType::U64 => quote!(#http_simple_type::Long).into(),
            PropertyType::I64 => quote!(#http_simple_type::Long).into(),
            PropertyType::F32 => quote!(#http_simple_type::Float).into(),
            PropertyType::F64 => quote!(#http_simple_type::Float).into(),
            PropertyType::USize => quote!(#http_simple_type::Long).into(),
            PropertyType::ISize => quote!(#http_simple_type::Long).into(),
            PropertyType::Bool => quote!(#http_simple_type::Boolean).into(),
            PropertyType::DateTime => quote!(#http_simple_type::DateTime).into(),
            PropertyType::Struct(..) => quote!(#http_simple_type::Binary).into(),
            _ => None,
        }
    }
}

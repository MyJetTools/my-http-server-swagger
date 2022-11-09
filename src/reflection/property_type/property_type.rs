use syn::TypePath;

use super::AsStr;

pub const U8: &str = "u8";
pub const I8: &str = "i8";
pub const U16: &str = "u16";
pub const I16: &str = "i16";
pub const U32: &str = "u32";
pub const I32: &str = "i32";
pub const U64: &str = "u64";
pub const I64: &str = "i64";
pub const F32: &str = "f32";
pub const F64: &str = "f64";
pub const U_SIZE: &str = "usize";
pub const I_SIZE: &str = "isize";
pub const BOOL: &str = "bool";
pub const STRING: &str = "String";
pub const DATE_TIME: &str = "DateTimeAsMicroseconds";
pub const FILE_CONTENT: &str = "FileContent";
pub const STR: &str = "&str";

pub enum PropertyType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    USize,
    ISize,
    String,
    Str,
    Bool,
    DateTime,
    FileContent,
    OptionOf(Box<PropertyType>),
    VecOf(Box<PropertyType>),
    Struct(String),
}

impl PropertyType {
    pub fn new(field: &syn::Field) -> Self {
        match &field.ty {
            syn::Type::Slice(_) => panic!("Slice type is not supported"),
            syn::Type::Array(_) => panic!("Array type is not supported"),
            syn::Type::Ptr(_) => panic!("Ptr type is not supported"),
            syn::Type::Reference(_) => PropertyType::Str,
            syn::Type::BareFn(_) => panic!("BareFn type is not supported"),
            syn::Type::Never(_) => panic!("Never type is not supported"),
            syn::Type::Tuple(_) => panic!("Tuple type is not supported"),
            syn::Type::Path(type_path) => {
                let type_as_string = super::utils::simple_type_to_string(type_path);
                return PropertyType::parse(type_as_string.as_str(), type_path);
            }
            syn::Type::TraitObject(_) => panic!("TraitObject type is not supported"),
            syn::Type::ImplTrait(_) => panic!("ImplTrait type is not supported"),
            syn::Type::Paren(_) => panic!("Paren type is not supported"),
            syn::Type::Group(_) => panic!("Group type is not supported"),
            syn::Type::Infer(_) => panic!("Infer type is not supported"),
            syn::Type::Macro(_) => panic!("Macro type is not supported"),
            syn::Type::Verbatim(_) => panic!("Verbatim type is not supported"),
            _ => panic!("{:?} type is not supported", &field.ty),
        }
    }

    pub fn parse(src: &str, type_path: &TypePath) -> Self {
        match src {
            U8 => PropertyType::U8,
            I8 => PropertyType::I8,
            U16 => PropertyType::U16,
            I16 => PropertyType::I16,
            U32 => PropertyType::U32,
            I32 => PropertyType::I32,
            U64 => PropertyType::U64,
            I64 => PropertyType::I64,
            F32 => PropertyType::F32,
            F64 => PropertyType::F64,
            U_SIZE => PropertyType::USize,
            I_SIZE => PropertyType::ISize,
            BOOL => PropertyType::Bool,
            STRING => PropertyType::String,
            DATE_TIME => PropertyType::DateTime,
            FILE_CONTENT => PropertyType::FileContent,
            "Option" => PropertyType::OptionOf(Box::new(super::utils::get_generic(type_path))),
            "Vec" => PropertyType::VecOf(Box::new(super::utils::get_generic(type_path))),
            _ => PropertyType::Struct(src.to_string()),
        }
    }

    pub fn as_str(&self) -> AsStr {
        match self {
            PropertyType::U8 => AsStr::create_as_str(U8),
            PropertyType::I8 => AsStr::create_as_str(I8),
            PropertyType::U16 => AsStr::create_as_str(U16),
            PropertyType::I16 => AsStr::create_as_str(I16),
            PropertyType::U32 => AsStr::create_as_str(U32),
            PropertyType::I32 => AsStr::create_as_str(I32),
            PropertyType::U64 => AsStr::create_as_str(U64),
            PropertyType::I64 => AsStr::create_as_str(I64),
            PropertyType::F32 => AsStr::create_as_str(F32),
            PropertyType::F64 => AsStr::create_as_str(F64),
            PropertyType::USize => AsStr::create_as_str(U_SIZE),
            PropertyType::ISize => AsStr::create_as_str(I_SIZE),
            PropertyType::String => AsStr::create_as_str(STRING),
            PropertyType::Str => AsStr::create_as_str(STR),
            PropertyType::Bool => AsStr::create_as_str(BOOL),
            PropertyType::DateTime => AsStr::create_as_str(DATE_TIME),
            PropertyType::FileContent => AsStr::create_as_str(FILE_CONTENT),
            PropertyType::OptionOf(generic_type) => {
                AsStr::create_as_string(format!("Option<{}>", generic_type.as_str()))
            }
            PropertyType::VecOf(generic_type) => {
                AsStr::create_as_string(format!("Vec<{}>", generic_type.as_str()))
            }
            PropertyType::Struct(name) => AsStr::create_as_str(name.as_str()),
        }
    }

    pub fn is_simple_type(&self) -> bool {
        match self {
            PropertyType::U8 => true,
            PropertyType::I8 => true,
            PropertyType::U16 => true,
            PropertyType::I16 => true,
            PropertyType::U32 => true,
            PropertyType::I32 => true,
            PropertyType::U64 => true,
            PropertyType::I64 => true,
            PropertyType::F64 => true,
            PropertyType::F32 => true,
            PropertyType::USize => true,
            PropertyType::ISize => true,
            PropertyType::String => true,
            PropertyType::DateTime => true,
            PropertyType::Str => false,
            PropertyType::Bool => true,
            PropertyType::FileContent => true,
            _ => false,
        }
    }

    pub fn get_swagger_simple_type(&self) -> Option<String> {
        use crate::consts::HTTP_SIMPLE_TYPE;

        match self {
            PropertyType::String => format!("{HTTP_SIMPLE_TYPE}::String").into(),
            PropertyType::Str => format!("{HTTP_SIMPLE_TYPE}::String").into(),
            PropertyType::U8 => format!("{HTTP_SIMPLE_TYPE}::Integer").into(),
            PropertyType::I8 => format!("{HTTP_SIMPLE_TYPE}::Integer").into(),
            PropertyType::U16 => format!("{HTTP_SIMPLE_TYPE}::Integer").into(),
            PropertyType::I16 => format!("{HTTP_SIMPLE_TYPE}::Integer").into(),
            PropertyType::U32 => format!("{HTTP_SIMPLE_TYPE}::Integer").into(),
            PropertyType::I32 => format!("{HTTP_SIMPLE_TYPE}::Integer").into(),
            PropertyType::U64 => format!("{HTTP_SIMPLE_TYPE}::Long").into(),
            PropertyType::I64 => format!("{HTTP_SIMPLE_TYPE}::Long").into(),
            PropertyType::F32 => format!("{HTTP_SIMPLE_TYPE}::Float").into(),
            PropertyType::F64 => format!("{HTTP_SIMPLE_TYPE}::Float").into(),
            PropertyType::USize => format!("{HTTP_SIMPLE_TYPE}::Long").into(),
            PropertyType::ISize => format!("{HTTP_SIMPLE_TYPE}::Long").into(),
            PropertyType::Bool => format!("{HTTP_SIMPLE_TYPE}::Boolean").into(),
            PropertyType::DateTime => format!("{HTTP_SIMPLE_TYPE}::DateTime").into(),
            PropertyType::FileContent => format!("{HTTP_SIMPLE_TYPE}::Binary").into(),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        if let PropertyType::String = self {
            return true;
        }

        false
    }

    pub fn is_boolean(&self) -> bool {
        if let PropertyType::Bool = self {
            return true;
        }

        false
    }

    pub fn is_date_time(&self) -> bool {
        if let PropertyType::DateTime = self {
            return true;
        }

        false
    }

    pub fn is_option(&self) -> bool {
        if let PropertyType::OptionOf(_) = self {
            return true;
        }

        false
    }

    pub fn is_vec(&self) -> bool {
        if let PropertyType::VecOf(_) = self {
            return true;
        }

        false
    }

    pub fn is_u8(&self) -> bool {
        if let PropertyType::U8 = self {
            return true;
        }

        false
    }
}

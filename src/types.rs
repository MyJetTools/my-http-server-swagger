use crate::consts::{
    HTTP_ARRAY_ELEMENT, HTTP_DATA_TYPE, HTTP_FIELD_TYPE, HTTP_SIMPLE_TYPE, NAME_SPACE,
};
use crate::reflection::PropertyType;

pub fn compile_http_field(
    name: &str,
    pt: &PropertyType,
    required: bool,
    default: Option<&str>,
) -> String {
    let default = if let Some(default) = default {
        format!("Some(\"{}\".to_string())", default)
    } else {
        "None".to_string()
    };

    format!(
        "{NAME_SPACE}::{HTTP_FIELD_TYPE}::new(\"{name}\", {data_type}, {required}, {default})",
        name = name,
        data_type = compile_data_type(pt, TypeIsWrappedTo::None),
        required = required,
        default = default
    )
}

pub fn compile_http_field_with_object(
    name: &str,
    body_type: &str,
    required: bool,
    default: Option<&str>,
) -> String {
    let default = if let Some(default) = default {
        format!("Some(\"{}\".to_string())", default)
    } else {
        "None".to_string()
    };

    format!(
        "{NAME_SPACE}::{HTTP_FIELD_TYPE}::new(\"{name}\", {data_type}, {required}, {default})",
        data_type = format!(
            "{body_type}::{fn_name}().into_http_data_type_object()",
            fn_name = crate::consts::FN_GET_HTTP_DATA_STRUCTURE
        ),
    )
}

enum TypeIsWrappedTo {
    None,
    Option,
    Vec,
}

fn compile_data_type(pt: &PropertyType, type_is_wrapped_to: TypeIsWrappedTo) -> String {
    if pt.is_option() {
        return compile_data_type(&pt.get_generic(), TypeIsWrappedTo::Option);
    }

    if pt.is_vec() {
        return compile_data_type(&pt.get_generic(), TypeIsWrappedTo::Vec);
    }

    if let Some(simple_type) = get_simple_type(pt.type_name.as_str()) {
        match type_is_wrapped_to {
            TypeIsWrappedTo::None => {
                return format!("{NAME_SPACE}::{HTTP_DATA_TYPE}::SimpleType({simple_type})",)
            }
            TypeIsWrappedTo::Option => {
                return format!("{NAME_SPACE}::{HTTP_DATA_TYPE}::SimpleType({simple_type})",)
            }
            TypeIsWrappedTo::Vec => {
                let result = format!(
                    "{NAME_SPACE}::{HTTP_DATA_TYPE}::ArrayOf({NAME_SPACE}::{HTTP_ARRAY_ELEMENT}::SimpleType({simple_type}))",
                );
                return result;
            }
        };
    }

    match type_is_wrapped_to {
        TypeIsWrappedTo::None => {
            return format!(
                "{}::{}().into_http_data_type_object()",
                pt.type_name,
                func_name = crate::consts::FN_GET_HTTP_DATA_STRUCTURE
            );
        }
        TypeIsWrappedTo::Option => {
            return format!(
                "{}::{}().into_http_data_type_object()",
                pt.type_name,
                func_name = crate::consts::FN_GET_HTTP_DATA_STRUCTURE
            );
        }
        TypeIsWrappedTo::Vec => {
            return format!(
                "{}::{}().into_http_data_type_array()",
                pt.type_name,
                func_name = crate::consts::FN_GET_HTTP_DATA_STRUCTURE
            );
        }
    }
}

fn get_simple_type(type_name: &str) -> Option<String> {
    match type_name {
        "String" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::String").into(),
        "u8" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::Integer").into(),
        "i8" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::Integer").into(),
        "u16" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::Integer").into(),
        "i16" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::Integer").into(),
        "u32" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::Integer").into(),
        "i32" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::Integer").into(),
        "u64" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::Long").into(),
        "i64" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::Long").into(),
        "usize" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::Long").into(),
        "isize" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::Long").into(),
        "bool" => format!("{NAME_SPACE}::{HTTP_SIMPLE_TYPE}::Boolean").into(),
        _ => None,
    }
}

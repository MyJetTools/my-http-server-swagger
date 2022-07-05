use crate::consts::{HTTP_ARRAY_ELEMENT, HTTP_DATA_TYPE, HTTP_FIELD_TYPE, NAME_SPACE};
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
    if let PropertyType::OptionOf(generic_type) = pt {
        return compile_data_type(generic_type.as_ref(), TypeIsWrappedTo::Option);
    }

    if let PropertyType::VecOf(generic_type) = pt {
        return compile_data_type(generic_type.as_ref(), TypeIsWrappedTo::Vec);
    }

    if let Some(simple_type) = pt.get_swagger_simple_type() {
        match type_is_wrapped_to {
            TypeIsWrappedTo::None => return format!("{HTTP_DATA_TYPE}::SimpleType({simple_type})",),

            TypeIsWrappedTo::Option => {
                return format!("{HTTP_DATA_TYPE}::SimpleType({simple_type})",)
            }
            TypeIsWrappedTo::Vec => {
                let result = format!(
                    "{HTTP_DATA_TYPE}::ArrayOf({HTTP_ARRAY_ELEMENT}::SimpleType({simple_type}))",
                );
                return result;
            }
        };
    }

    match type_is_wrapped_to {
        TypeIsWrappedTo::None => {
            return format!(
                "{}::{}().into_http_data_type_object()",
                pt.as_str(),
                func_name = crate::consts::FN_GET_HTTP_DATA_STRUCTURE
            );
        }
        TypeIsWrappedTo::Option => {
            return format!(
                "{}::{}().into_http_data_type_object()",
                pt.as_str(),
                func_name = crate::consts::FN_GET_HTTP_DATA_STRUCTURE
            );
        }
        TypeIsWrappedTo::Vec => {
            return format!(
                "{}::{}().into_http_data_type_array()",
                pt.as_str(),
                func_name = crate::consts::FN_GET_HTTP_DATA_STRUCTURE
            );
        }
    }
}

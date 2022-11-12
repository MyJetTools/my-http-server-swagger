use crate::consts::{
    HTTP_ARRAY_ELEMENT, HTTP_DATA_TYPE, HTTP_FIELD_TYPE, HTTP_SIMPLE_TYPE, NAME_SPACE,
};
use crate::input_models::input_fields::InputFieldSource;
use crate::reflection::PropertyType;

pub fn compile_http_field(
    name: &str,
    pt: &PropertyType,
    required: bool,
    default: Option<&str>,
    src: Option<&InputFieldSource>,
) -> String {
    let default = if let Some(default) = default {
        format!("Some(\"{}\".to_string())", default)
    } else {
        "None".to_string()
    };

    let data_type = if let Some(src) = src {
        if src.is_body_file() {
            format!("{HTTP_DATA_TYPE}::SimpleType({HTTP_SIMPLE_TYPE}::Binary)",)
        } else {
            compile_data_type(pt, TypeIsWrappedTo::None)
        }
    } else {
        compile_data_type(pt, TypeIsWrappedTo::None)
    };

    format!(
        "{HTTP_FIELD_TYPE}::new(\"{name}\", {data_type}, {required}, {default})",
        name = name,
        data_type = data_type,
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
        "{HTTP_FIELD_TYPE}::new(\"{name}\", {data_type}, {required}, {default})",
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
                crate::consts::FN_GET_HTTP_DATA_STRUCTURE
            );
        }
        TypeIsWrappedTo::Option => {
            return format!(
                "{}::{}().into_http_data_type_object()",
                pt.as_str(),
                crate::consts::FN_GET_HTTP_DATA_STRUCTURE
            );
        }
        TypeIsWrappedTo::Vec => {
            return format!(
                "{}::{}().into_http_data_type_array()",
                pt.as_str(),
                crate::consts::FN_GET_HTTP_DATA_STRUCTURE
            );
        }
    }
}

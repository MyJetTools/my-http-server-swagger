use crate::consts::{HTTP_ARRAY_ELEMENT, HTTP_DATA_TYPE, HTTP_FIELD_TYPE, HTTP_SIMPLE_TYPE};
use crate::input_models::input_fields::InputFieldSource;
use crate::reflection::PropertyType;

pub fn compile_http_field(
    result: &mut String,
    name: &str,
    pt: &PropertyType,
    required: bool,
    default: Option<&str>,
    src: Option<&InputFieldSource>,
) {
    result.push_str(HTTP_FIELD_TYPE);
    result.push_str("::new(\"");
    result.push_str(name);

    result.push_str("\", ");

    if let Some(src) = src {
        if src.is_body_file() {
            result.push_str(HTTP_DATA_TYPE);
            result.push_str("::SimpleType(");

            result.push_str(HTTP_SIMPLE_TYPE);
            result.push_str("::Binary)");
        } else {
            compile_data_type(result, pt, TypeIsWrappedTo::None);
        }
    } else {
        compile_data_type(result, pt, TypeIsWrappedTo::None);
    }

    result.push_str(", ");

    write_bool_value(result, required);

    result.push_str(", ");

    write_optional_value(result, default);

    result.push(')');
}

pub fn compile_http_field_with_object(
    result: &mut String,
    name: &str,
    body_type: &str,
    required: bool,
    default: Option<&str>,
) {
    result.push_str(HTTP_FIELD_TYPE);

    result.push_str("::new(\"");
    result.push_str(name);

    result.push_str("\", ");

    result.push_str(body_type);
    result.push_str("::");
    result.push_str(crate::consts::FN_GET_HTTP_DATA_STRUCTURE);
    result.push_str("().into_http_data_type_object(), ");

    write_bool_value(result, required);

    result.push_str(", ");

    write_optional_value(result, default);

    result.push_str(")");
}

enum TypeIsWrappedTo {
    None,
    Option,
    Vec,
}

fn compile_data_type(result: &mut String, pt: &PropertyType, type_is_wrapped_to: TypeIsWrappedTo) {
    if let PropertyType::OptionOf(generic_type) = pt {
        compile_data_type(result, generic_type.as_ref(), TypeIsWrappedTo::Option);
        return;
    }

    if let PropertyType::VecOf(generic_type) = pt {
        compile_data_type(result, generic_type.as_ref(), TypeIsWrappedTo::Vec);
        return;
    }

    if let Some(simple_type) = pt.get_swagger_simple_type() {
        match type_is_wrapped_to {
            TypeIsWrappedTo::None => {
                result.push_str(HTTP_DATA_TYPE);
                result.push_str("::SimpleType(");
                result.push_str(simple_type.as_str());
                result.push_str(")");
                return;
            }

            TypeIsWrappedTo::Option => {
                result.push_str(HTTP_DATA_TYPE);
                result.push_str("::SimpleType(");
                result.push_str(simple_type.as_str());
                result.push_str(")");
                return;
            }
            TypeIsWrappedTo::Vec => {
                result.push_str(HTTP_DATA_TYPE);
                result.push_str("::ArrayOf(");
                result.push_str(HTTP_ARRAY_ELEMENT);
                result.push_str("::SimpleType(");
                result.push_str(simple_type.as_str());
                result.push_str("))");
                return;
            }
        };
    }

    match type_is_wrapped_to {
        TypeIsWrappedTo::None => {
            result.push_str(pt.as_str().as_str());
            result.push_str("::");
            result.push_str(crate::consts::FN_GET_HTTP_DATA_STRUCTURE);
            result.push_str("().into_http_data_type_object()");
            return;
        }
        TypeIsWrappedTo::Option => {
            result.push_str(pt.as_str().as_str());
            result.push_str("::");
            result.push_str(crate::consts::FN_GET_HTTP_DATA_STRUCTURE);
            result.push_str("().into_http_data_type_object()");
            return;
        }
        TypeIsWrappedTo::Vec => {
            result.push_str(pt.as_str().as_str());
            result.push_str("::");
            result.push_str(crate::consts::FN_GET_HTTP_DATA_STRUCTURE);
            result.push_str("().into_http_data_type_array()");
            return;
        }
    }
}

fn write_optional_value(result: &mut String, value: Option<&str>) {
    if let Some(value) = value {
        result.push_str("Some(\"");
        result.push_str(value);
        result.push_str("\".to_string()");
    } else {
        result.push_str("None");
    };
}

fn write_bool_value(result: &mut String, value: bool) {
    if value {
        result.push_str("true");
    } else {
        result.push_str("false");
    };
}

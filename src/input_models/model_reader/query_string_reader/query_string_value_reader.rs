use crate::{
    input_models::input_fields::{InputField, InputFields},
    reflection::PropertyType,
};

use super::SourceToRead;

pub fn read_required_struct_parameter(
    source_to_read: &SourceToRead,
    input_field: &InputField,
) -> String {
    format!(
        "{src}.get_required_parameter(\"{http_name}\")?,\n",
        src = source_to_read.get_source_variable(),
        http_name = input_field.name()
    )
}

pub fn read_string_parameter_with_default_value(
    source_to_read: &SourceToRead,
    input_field: &InputField,
    default: &str,
) -> String {
    let optional_string = generate_read_optional_string_parameter(source_to_read, input_field);
    option_of_str_to_default(optional_string.as_str(), default)
}

pub fn read_system_parameter_with_default_value(
    source_to_read: &SourceToRead,
    input_field: &InputField,
    default: &str,
) -> String {
    let optional_string = generate_read_optional_parameter(source_to_read, input_field);
    option_to_system_default(optional_string.as_str(), default)
}

pub fn read_boolean_with_default_value(
    source_to_read: &SourceToRead,
    input_field: &InputField,
    default: &str,
) -> String {
    format!(
        r###"
             if let Some(value) = {src}.get_optional("{http_name}"){{
                value.as_bool()?
            }}else{{
               {default}
            }}
        "###,
        http_name = input_field.name(),
        src = source_to_read.get_source_variable()
    )
}

pub fn read_struct_parameter_with_default_value(
    source_to_read: &SourceToRead,
    input_field: &InputField,
    default: &str,
) -> String {
    format!(
        r###"
             if let Some(value) = {src}.get_optional("{http_name}"){{
               value.parse::<{type_name}>()?
            }}else{{
               "{default}".parse::<{type_name}>()?
            }}
        "###,
        type_name = input_field.property.ty.as_str(),
        http_name = input_field.name(),
        src = source_to_read.get_source_variable()
    )
}

pub fn read_optional_str_parameter(
    source_to_read: &SourceToRead,
    input_field: &InputField,
) -> String {
    format!(
        "{src}.get_optional(\"{http_name}\")",
        src = source_to_read.get_source_variable(),
        http_name = input_field.name()
    )
}

pub fn read_optional_parameter(source_to_read: &SourceToRead, input_field: &InputField) -> String {
    format!(
        "{src}.get_optional(\"{http_name}\")",
        src = source_to_read.get_source_variable(),
        http_name = input_field.name()
    )
}

fn generate_read_optional_string_parameter(
    source_to_read: &SourceToRead,
    input_field: &InputField,
) -> String {
    format!(
        "{src}.get_optional(\"{http_name}\")",
        src = source_to_read.get_source_variable(),
        http_name = input_field.name()
    )
}

fn generate_read_optional_parameter(
    source_to_read: &SourceToRead,
    input_field: &InputField,
) -> String {
    format!(
        "{src}.get_optional(\"{http_name}\")",
        src = source_to_read.get_source_variable(),
        http_name = input_field.name()
    )
}

fn option_of_str_to_default(expr: &str, default: &str) -> String {
    format!(
        r###"
        if let Some(value) = {expr}{{
            value.to_string()
        }}else{{
            "{default}".to_string()
        }}
    "###,
        expr = expr,
        default = default
    )
}

fn option_to_system_default(expr: &str, default: &str) -> String {
    format!(
        r###"
        if let Some(value) = {expr}{{
            value
        }}else{{
            {default}
        }}
    "###,
        expr = expr,
        default = default
    )
}

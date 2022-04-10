use crate::{input_models::input_fields::InputField, reflection::PropertyType};

pub enum SourceToRead {
    FormData,
    QueryString,
}

impl SourceToRead {
    pub fn get_source_variable(&self) -> &str {
        match self {
            SourceToRead::FormData => "form_data",
            SourceToRead::QueryString => "query_string",
        }
    }
}

pub fn read_string_parameter_with_default_value(
    source_to_read: &SourceToRead,
    input_field: &InputField,
    default: &str,
) -> String {
    let optional_string = generate_read_optional_string_parameter(source_to_read, input_field);
    let get_value = option_of_str_to_default(optional_string.as_str(), default);
    compile_read_line(input_field, get_value.as_str())
}

pub fn read_system_parameter_with_default_value(
    source_to_read: &SourceToRead,
    input_field: &InputField,
    default: &str,
) -> String {
    let optional_string = generate_read_optional_parameter(source_to_read, input_field);
    let get_value = option_to_system_default(optional_string.as_str(), default);
    compile_read_line(input_field, get_value.as_str())
}

pub fn read_parameter_with_default_value(
    source_to_read: &SourceToRead,
    input_field: &InputField,
    default: &str,
) -> String {
    let optional_string = generate_read_optional_string_parameter(source_to_read, input_field);
    let get_value = option_to_default(optional_string.as_str(), default, &input_field.property.ty);
    compile_read_line(input_field, get_value.as_str())
}

pub fn read_optional_string_parameter(
    source_to_read: &SourceToRead,
    input_field: &InputField,
) -> String {
    let get_optional_value = format!(
        "{src}.get_optional_string_parameter(\"{http_name}\")",
        src = source_to_read.get_source_variable(),
        http_name = input_field.name()
    );

    let get_value = option_of_str_to_option_of_string(get_optional_value.as_str());
    compile_read_line(input_field, get_value.as_str())
}

pub fn read_optional_str_parameter(
    source_to_read: &SourceToRead,
    input_field: &InputField,
) -> String {
    let get_optional_str_value = format!(
        "{src}.get_optional_string_parameter(\"{http_name}\")",
        src = source_to_read.get_source_variable(),
        http_name = input_field.name()
    );

    compile_read_line(input_field, get_optional_str_value.as_str())
}

pub fn read_optional_parameter(source_to_read: &SourceToRead, input_field: &InputField) -> String {
    let get_value = format!(
        "{src}.get_optional_parameter(\"{http_name}\")",
        src = source_to_read.get_source_variable(),
        http_name = input_field.name()
    );

    compile_read_line(input_field, get_value.as_str())
}

pub fn read_from_headers(input_field: &InputField) -> String {
    if input_field.required() {
        if input_field.property.ty.is_string() {
            return format!(
                "{struct_field_name}: ctx.request.get_required_header(\"{header_name}\")?.to_string(),\n",
                struct_field_name = input_field.struct_field_name(),
                header_name = input_field.name().to_lowercase()
            );
        }
        if input_field.property.ty.is_str() {
            return format!(
                "{struct_field_name}: ctx.request.get_required_header(\"{header_name}\")?,\n",
                struct_field_name = input_field.struct_field_name(),
                header_name = input_field.name().to_lowercase()
            );
        } else {
            panic!("Header can only be read to String or str typed property");
        }
    }

    if let PropertyType::OptionOf(inner_generic) = &input_field.property.ty {
        if inner_generic.is_string() {
            let get_optional_header = format!(
                "ctx.request.get_optional_header(\"{header_name}\")",
                header_name = input_field.name().to_lowercase()
            );

            return format!(
                "{struct_field_name}: {str_converions},\n",
                struct_field_name = input_field.struct_field_name(),
                str_converions = option_of_str_to_option_of_string(get_optional_header.as_str())
            );
        }

        if inner_generic.is_string() {
            let get_optional_header = format!(
                "ctx.request.get_optional_header(\"{header_name}\")",
                header_name = input_field.name().to_lowercase()
            );

            return format!(
                "{struct_field_name}: {str_converions},\n",
                struct_field_name = input_field.struct_field_name(),
                str_converions = option_of_str_to_option_of_string(get_optional_header.as_str())
            );
        }

        panic!("Header can only be read to String or str typed property");
    }

    panic!("Non required filed must be optional");
}

fn compile_read_line(input_field: &InputField, reading_line: &str) -> String {
    format!(
        "{struct_field_name}: {reading_line},\n",
        struct_field_name = input_field.struct_field_name(),
        reading_line = reading_line
    )
}

fn generate_read_optional_string_parameter(
    source_to_read: &SourceToRead,
    input_field: &InputField,
) -> String {
    format!(
        "{src}.get_optional_string_parameter(\"{http_name}\")",
        src = source_to_read.get_source_variable(),
        http_name = input_field.name()
    )
}

fn generate_read_optional_parameter(
    source_to_read: &SourceToRead,
    input_field: &InputField,
) -> String {
    format!(
        "{src}.get_optional_parameter(\"{http_name}\")",
        src = source_to_read.get_source_variable(),
        http_name = input_field.name()
    )
}

pub fn option_of_str_to_option_of_string(expr: &str) -> String {
    format!(
        r###"
        if let Some(value) = {expr}{{
            Some(value.to_string())
        }}else{{
            None
        }}
    "###,
        expr = expr,
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

fn option_to_default(expr: &str, default: &str, ty: &PropertyType) -> String {
    format!(
        r###"
        if let Some(value) = {expr}{{
            {ty}::{fn_parse_str}(value)?
        }}else{{
            {ty}::{fn_parse_str}("{default}")?
        }}
    "###,
        expr = expr,
        default = default,
        ty = ty.as_str(),
        fn_parse_str = crate::consts::FN_PARSE_STR
    )
}

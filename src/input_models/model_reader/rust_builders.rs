use crate::{
    input_models::input_fields::{InputField, InputFields},
    reflection::PropertyType,
};

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

pub fn init_header_variables(result: &mut String, input_fields: &InputFields) {
    for input_field_header in input_fields.get_from_header_elements() {
        let mut valid_type = false;

        if input_field_header.property.ty.is_string() {
            valid_type = true;
        }

        if let PropertyType::OptionOf(inner_generic) = &input_field_header.property.ty {
            if inner_generic.is_string() {
                valid_type = true;
            }
        }

        if !valid_type {
            panic!(
                "Can not read {} type to from header to property {}",
                input_field_header.property.ty.as_str(),
                input_field_header.struct_field_name()
            );
        }

        let line = if input_field_header.required() {
            format!("let {field_name}_header = ctx.request.get_required_header(\"{header_key}\")?.to_string();\n", field_name=input_field_header.struct_field_name(), header_key=input_field_header.name())
        } else {
            let reading_command = format!(
                "ctx.request.get_optional_header(\"{header_key}\")",
                header_key = input_field_header.name()
            );

            let reading_command =
                super::rust_builders::option_of_str_to_option_of_string(reading_command.as_str());

            format!(
                "let {field_name}_header = {reading_command};\n",
                field_name = input_field_header.struct_field_name(),
            )
        };

        result.push_str(line.as_str());
    }
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

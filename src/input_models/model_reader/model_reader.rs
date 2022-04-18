use crate::input_models::input_fields::{InputField, InputFieldSource, InputFields};

use super::query_string_value_reader::SourceToRead;

pub fn generate(name: &str, input_fields: &InputFields) -> String {
    let mut result = String::new();

    add_init_lines(&mut result, input_fields);

    if input_fields.has_query() {
        super::query_string::generate_init_line(
            &mut result,
            input_fields,
            SourceToRead::QueryString,
        );
    }

    if input_fields.has_form_data() {
        result.push_str("let body = ctx.request.get_body()?;\n");
    }

    result.push_str("Ok(");
    result.push_str(name);
    result.push('{');

    for input_field in &input_fields.fields {
        match &input_field.src {
            InputFieldSource::Query => {
                result.push_str(input_field.property.name.as_str());
                result.push(',');
            }
            InputFieldSource::Path => {
                let line_to_add = if input_field.required() {
                    format!(
                        "{}: request.get_value_from_path(\"{}\")?.to_string(),",
                        input_field.struct_field_name(),
                        input_field.name()
                    )
                } else {
                    format!(
                        "{}: request.get_value_from_path_optional_as_string(\"{}\")?,",
                        input_field.struct_field_name(),
                        input_field.name()
                    )
                };

                result.push_str(line_to_add.as_str());
            }
            InputFieldSource::Header => {
                let line_to_add = format!(
                    "{field_name}:{field_name}_header,",
                    field_name = input_field.struct_field_name()
                );
                result.push_str(line_to_add.as_str());
            }
            InputFieldSource::Body => {
                super::read_body::generate_read_body(&mut result, input_field);
            }
            InputFieldSource::Form => {
                result.push_str(input_field.property.name.as_str());
                result.push(',');
            }
        }
    }

    if let Some(body_field) = input_fields.get_body_field() {
        add_reading_body(&mut result, body_field);
    }

    result.push_str("})");

    result
}

fn add_init_lines(result: &mut String, input_fields: &InputFields) {
    super::query_string_value_reader::init_header_variables(result, input_fields);
}

fn add_reading_body(result: &mut String, body_field: &InputField) {
    result.push_str(format!("{}: body,\n", body_field.struct_field_name()).as_str());
}

/*
fn read_with_default(
    source_to_read: &SourceToRead,
    input_field: &InputField,
    default: &str,
) -> String {
    if input_field.property.ty.is_string() {
        return super::query_string_value_reader::read_string_parameter_with_default_value(
            source_to_read,
            input_field,
            default,
        );
    }
    if input_field.property.ty.is_simple_type() {
        return super::query_string_value_reader::read_system_parameter_with_default_value(
            source_to_read,
            input_field,
            default,
        );
    }

    return super::query_string_value_reader::read_struct_parameter_with_default_value(
        source_to_read,
        input_field,
        default,
    );
}
 */

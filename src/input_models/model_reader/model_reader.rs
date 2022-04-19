use crate::input_models::input_fields::{InputFieldSource, InputFields};

use super::query_string_reader::SourceToRead;

pub fn generate(name: &str, input_fields: &InputFields) -> String {
    let mut result = String::new();

    add_init_lines(&mut result, input_fields);

    if input_fields.has_query() {
        super::query_string_reader::generate_as_reading(
            &mut result,
            input_fields,
            SourceToRead::QueryString,
        );
    }

    if input_fields.has_form_data() {
        super::query_string_reader::generate_as_reading(
            &mut result,
            input_fields,
            SourceToRead::FormData,
        );
    }

    result.push_str("Ok(");
    result.push_str(name);
    result.push('{');

    for input_field in &input_fields.fields {
        match &input_field.src {
            InputFieldSource::Query => {
                result.push_str(input_field.struct_field_name());
                result.push(',');
            }
            InputFieldSource::Path => {
                let line_to_add = if input_field.required() {
                    format!(
                        "{}: request.get_value_from_path(\"{}\")?.to_string()",
                        input_field.struct_field_name(),
                        input_field.name()
                    )
                } else {
                    format!(
                        "{}: request.get_value_from_path_optional_as_string(\"{}\")?",
                        input_field.struct_field_name(),
                        input_field.name()
                    )
                };

                result.push_str(line_to_add.as_str());
            }
            InputFieldSource::Header => {
                result.push_str(input_field.struct_field_name());
                result.push(',');
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

    result.push_str("})");

    result
}

fn add_init_lines(result: &mut String, input_fields: &InputFields) {
    super::header_reader::init_header_variables(result, input_fields)
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

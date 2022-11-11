use crate::input_models::input_fields::{InputFieldSource, InputFields};

pub fn generate(result: &mut String, name: &str, input_fields: &InputFields) {
    if input_fields.has_query() {
        super::not_body_reading::generate(result, input_fields);
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
                result.push_str(input_field.struct_field_name());
                result.push(',');
            }
            InputFieldSource::Header => {
                result.push_str(input_field.struct_field_name());
                result.push(',');
            }
            InputFieldSource::Body => { /*  Skipping on first go*/ }
            InputFieldSource::Form => { /*  Skipping on first go*/ }
            InputFieldSource::BodyFile => { /*  Skipping on first go*/ }
        }
    }

    result.push_str("})");
}

fn add_init_lines(result: &mut String, input_fields: &InputFields) {
    super::header_reader::init_header_variables(result, input_fields)
}

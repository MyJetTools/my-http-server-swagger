use crate::input_models::input_fields::{InputFieldSource, InputFields};

pub fn generate(result: &mut String, name: &str, input_fields: &InputFields) {
    if input_fields.has_query() {
        super::not_body_reading::generate(result, input_fields);
    }

    if input_fields.has_body_reading_data() {
        result.push_str("let __body = ctx.request.receive_body().await?;");
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
            InputFieldSource::Body => {
                result.push_str(input_field.struct_field_name());
                result.push_str("__body.get_body(),");
            }
            InputFieldSource::Form => { /*  Skipping on first go*/ }
            InputFieldSource::BodyFile => {
                result.push_str(input_field.struct_field_name());
                result.push_str("__body.get_body(),");
            }
        }
    }

    result.push_str("})");
}

fn add_init_lines(result: &mut String, input_fields: &InputFields) {
    super::header_reader::init_header_variables(result, input_fields)
}

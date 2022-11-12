use crate::input_models::input_fields::{InputFieldSource, InputFields};

pub fn generate(result: &mut String, name: &str, input_fields: &InputFields) {
    input_fields.check_types_of_field();

    if input_fields.has_query() {
        super::generate_read_not_body(result, input_fields);
    }

    if input_fields.has_body_reading_data() {
        if input_fields.has_body_file() || input_fields.has_body_to_vec() {
            result.push_str("let __body = ctx.request.receive_body().await?;");
        } else {
            result.push_str("let __body = ctx.request.get_body().await?;");
        }
    }

    if input_fields.has_body_data_to_read() {
        super::generate_read_body(result, input_fields);
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
                if input_field.is_body_to_vec() {
                    result.push_str(input_field.struct_field_name());
                    result.push_str(": __body.get_body(),");
                } else {
                    result.push_str(input_field.struct_field_name());
                    result.push_str(",");
                }
            }
            InputFieldSource::FormData => { /*  Skipping on first go*/ }
            InputFieldSource::BodyFile => {
                result.push_str(input_field.struct_field_name());
                result.push_str(": __body.get_body(),");
            }
        }
    }

    result.push_str("})");
}

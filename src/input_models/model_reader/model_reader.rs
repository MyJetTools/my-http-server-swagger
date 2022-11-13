use crate::input_models::input_fields::{BodyDataToReader, InputFieldSource, InputFields};

pub fn generate(result: &mut String, name: &str, input_fields: &InputFields) {
    input_fields.check_types_of_field();

    if input_fields.has_no_body_data_to_read() {
        super::generate_read_not_body(result, input_fields);
    }

    if let Some(body_data_reader_type) = input_fields.has_body_data_to_read() {
        match body_data_reader_type {
            BodyDataToReader::FormData => {
                result.push_str("let __body = ctx.request.get_body().await?;");
                super::generate_read_body(
                    result,
                    input_fields,
                    " = __body.get_body_form_data()?;",
                    |f| f.src_is_form_data(),
                );
            }
            BodyDataToReader::BodyFile => {
                result.push_str("let __body = ctx.request.receive_body().await?;");
            }
            BodyDataToReader::RawBodyToVec => {
                result.push_str("let __body = ctx.request.receive_body().await?;");
            }
            BodyDataToReader::BodyModel => {
                result.push_str("let __body = ctx.request.get_body().await?;");
                super::generate_read_body(
                    result,
                    input_fields,
                    " = __body.get_body_data_reader()?;",
                    |f| f.src_is_body(),
                );
            }
        }
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
            InputFieldSource::FormData => {
                result.push_str(input_field.struct_field_name());
                result.push(',');
            }
            InputFieldSource::BodyFile => {
                result.push_str(input_field.struct_field_name());
                result.push_str(": __body.get_body(),");
            }
        }
    }

    result.push_str("})");
}

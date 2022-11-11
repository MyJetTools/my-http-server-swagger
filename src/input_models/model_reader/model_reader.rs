use crate::input_models::input_fields::{InputFieldSource, InputFields};

pub fn generate(result: &mut String, name: &str, input_fields: &InputFields) {
    add_init_lines(result, input_fields);

    if input_fields.has_query() {
        super::query_string_reader::generate_reading_from_query_string(result, input_fields);
    }
    /*
       if input_fields.has_form_data() {
           super::query_string_reader::generate_as_reading(
               &mut result,
               input_fields,
               SourceToRead::FormData,
           );
       }
    */
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
                result.push_str("http_route.get_value(\"");
                result.push_str(input_field.name());
                result.push_str("\")?.try_into()?,");
            }
            InputFieldSource::Header => {
                result.push_str(input_field.struct_field_name());
                result.push(',');
            }
            InputFieldSource::Body => {}
            InputFieldSource::Form => {}
        }
    }

    result.push_str("})");
}

fn add_init_lines(result: &mut String, input_fields: &InputFields) {
    super::header_reader::init_header_variables(result, input_fields)
}

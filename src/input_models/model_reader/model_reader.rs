use crate::{
    input_models::input_fields::{InputField, InputFieldSource, InputFields},
    reflection::PropertyType,
};

pub fn generate(name: &str, input_fields: &InputFields) -> String {
    let mut result = String::new();

    add_init_lines(&mut result, input_fields);

    if input_fields.has_query() {
        result.push_str("let query_string = ctx.request.get_query_string()?;\n");
    }

    if let Some(form_data) = input_fields.get_form_data() {
        if let PropertyType::VecOf(inner_generic) = &form_data.property.ty {
            if inner_generic.is_u8() {
                result.push_str("let body = ctx.request.get_body().await?;\n");
            } else {
                result.push_str("let body = ctx.request.get_body_as_json().await?;\n");
            }
        } else {
            result.push_str("let body = ctx.request.get_body_as_json().await?;\n");
        }
    }

    result.push_str("Ok(");
    result.push_str(name);
    result.push('{');

    for input_field in &input_fields.fields {
        match &input_field.src {
            InputFieldSource::Query => {
                let line_to_add = build_reading(input_field, false);
                result.push_str(line_to_add.as_str());
            }
            InputFieldSource::Path => {
                let line_to_add = if input_field.required() {
                    format!(
                        "{}: ctx.request.get_value_from_path(\"{}\")?.to_string(),",
                        input_field.struct_field_name(),
                        input_field.name()
                    )
                } else {
                    format!(
                        "{}: ctx.request.get_value_from_path_optional_as_string(\"{}\")?,",
                        input_field.struct_field_name(),
                        input_field.name()
                    )
                };

                result.push_str(line_to_add.as_str());
            }
            InputFieldSource::Header => {
                let line_to_add = super::rust_builders::read_from_headers(input_field);
                result.push_str(line_to_add.as_str());
            }
            InputFieldSource::Body => {}
            InputFieldSource::Form => {
                let line_to_add = build_reading(input_field, true);
                result.push_str(line_to_add.as_str());
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
    if input_fields.has_query() {
        result.push_str("ctx.request.init_query_string()?;\n");
    }

    if input_fields.has_form_data() {
        result.push_str("ctx.request.init_form_data().await?;\n");
    }

    if input_fields.has_body_data() {
        result.push_str("ctx.request.init_body().await?;\n");
    }
}

fn add_reading_body(result: &mut String, body_field: &InputField) {
    result.push_str(format!("{}: body,\n", body_field.struct_field_name()).as_str());
}

fn build_reading(input_field: &InputField, form_data: bool) -> String {
    if let Some(default) = input_field.default() {
        if input_field.property.ty.is_option() {
            panic!("It does not make sence to have default value and Option type");
        }

        return read_with_default(form_data, input_field, default);
    }

    if input_field.required() {
        return read_required(form_data, input_field);
    }

    if let PropertyType::OptionOf(inner_generic) = &input_field.property.ty {
        if inner_generic.is_string() {
            return super::rust_builders::read_optional_string_parameter(form_data, input_field);
        } else if inner_generic.is_str() {
            return super::rust_builders::read_optional_str_parameter(form_data, input_field);
        } else {
            return super::rust_builders::read_optional_parameter(form_data, input_field);
        }
    }

    panic!("Non Required type must be Option");
}

fn read_with_default(form_data: bool, input_field: &InputField, default: &str) -> String {
    if input_field.property.ty.is_string() {
        return super::rust_builders::read_string_parameter_with_default_value(
            form_data,
            input_field,
            default,
        );
    }
    if input_field.property.ty.is_simple_type() {
        return super::rust_builders::read_system_parameter_with_default_value(
            form_data,
            input_field,
            default,
        );
    } else {
        return super::rust_builders::read_parameter_with_default_value(
            form_data,
            input_field,
            default,
        );
    }
}

fn read_required(form_data: bool, input_field: &InputField) -> String {
    let src = super::rust_builders::get_source_to_read(form_data);
    if input_field.property.ty.is_string() {
        format!(
            "{struct_field_name}: {src}.get_required_string_parameter(\"{http_name}\")?.to_string(),\n",
            struct_field_name = input_field.struct_field_name(),
            src = src,
            http_name = input_field.name()
        )
    } else {
        format!(
            "{struct_field_name}: {src}.get_required_parameter(\"{http_name}\")?,\n",
            struct_field_name = input_field.struct_field_name(),
            src = src,
            http_name = input_field.name()
        )
    }
}

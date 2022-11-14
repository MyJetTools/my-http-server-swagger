use crate::{
    input_models::input_fields::{InputField, InputFieldSource, InputFields},
    reflection::PropertyType,
};

const DATA_SRC: &str = "__query_string";

pub fn generate_read_not_body(result: &mut String, input_fields: &InputFields) {
    let mut validation: Option<String> = None;

    result.push_str("let ");
    generate_init_fields(result, input_fields);

    result.push_str("={\n");

    result.push_str("let ");
    result.push_str(DATA_SRC);

    result.push_str(" = ctx.request.get_query_string()?;\n");

    for input_field in &input_fields.fields {
        if input_field.is_reading_from_body() {
            continue;
        }

        result.push_str("let ");
        result.push_str(input_field.struct_field_name());
        result.push_str(" = ");

        match &input_field.property.ty {
            PropertyType::FileContent => {}
            PropertyType::OptionOf(_) => {
                result.push_str("if let Some(value) = ");
                result.push_str(DATA_SRC);
                result.push_str(".get_optional(\"");
                result.push_str(input_field.name());
                result.push_str("\"){");
                result.push_str(
                    "let value = my_http_server::InputParamValue::from(value);Some(value.try_into()?)}else{None};",
                );
            }
            PropertyType::VecOf(sub_type) => {
                if sub_type.is_string() {
                    result.push_str(DATA_SRC);
                    result.push_str(".get_vec_of_string(\"");
                    result.push_str(input_field.name());
                    result.push_str("\");");
                } else {
                    result.push_str(DATA_SRC);
                    result.push_str(".get_vec(\"");
                    result.push_str(input_field.name());
                    result.push_str("\");");
                }
            }
            PropertyType::Struct(_) => {}
            _ => {
                generate_reading_required(result, input_field);
            }
        }

        if let Some(validator) = input_field.validator() {
            if validation.is_none() {
                validation = Some(String::new());
            }
            validation.as_mut().unwrap().push_str(validator);
            validation.as_mut().unwrap().push_str("(ctx, &");
            validation
                .as_mut()
                .unwrap()
                .push_str(input_field.struct_field_name());
            validation.as_mut().unwrap().push_str(")?;\n");
        }
    }

    if let Some(validation) = validation {
        result.push_str(validation.as_str());
    }

    generate_init_fields(result, input_fields);
    result.push_str("};\n");
}

fn generate_reading_required(result: &mut String, input_field: &InputField) {
    match input_field.src {
        InputFieldSource::Query => {
            result.push_str("my_http_server::InputParamValue::from(");
            result.push_str(DATA_SRC);
            result.push_str(".get_required(\"");
            result.push_str(input_field.name());
            result.push_str("\")?).try_into()?;");
        }
        InputFieldSource::Path => {
            result.push_str("http_route.get_value(&ctx.request.http_path, \"");
            result.push_str(input_field.name());
            result.push_str("\")?.try_into()?;");
        }
        InputFieldSource::Header => {
            result.push_str("ctx.request.get_required_header(\"");
            result.push_str(input_field.name());
            result.push_str("\")?.try_into()?;");
        }
        InputFieldSource::Body => {
            panic!("Bug. Should not read Body at generate_reading_required");
        }
        InputFieldSource::FormData => {
            panic!("Bug. Should not read Form at generate_reading_required");
        }
        InputFieldSource::BodyFile => {
            panic!("Bug. Should not read BodyFile at generate_reading_required");
        }
    }
}

fn generate_init_fields(result: &mut String, input_fields: &InputFields) {
    let amount = input_fields
        .fields
        .iter()
        .filter(|f| !f.is_reading_from_body())
        .count();

    if amount > 1 {
        result.push('(');
    }

    let mut no = 0;
    for input_field in &input_fields.fields {
        if !input_field.is_reading_from_body() {
            if no > 0 {
                result.push(',');
            }
            result.push_str(input_field.property.name.as_str());
            no += 1;
        }
    }

    if amount > 1 {
        result.push(')');
    }
}

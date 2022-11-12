use crate::{
    input_models::input_fields::{InputField, InputFieldSource, InputFields},
    reflection::PropertyType,
};

//const DATA_SRC: &str = "__body_reader";
pub fn generate_read_body<TInputFiler: Fn(&InputField) -> bool>(
    result: &mut String,
    input_fields: &InputFields,
    src_reader: &str,
    filter: TInputFiler,
) {
    let mut validation: Option<String> = None;

    result.push_str("let ");
    generate_init_fields(result, input_fields, &filter);

    result.push_str("={\n");

    result.push_str("let ");
    result.push_str(src_reader);

    result.push_str(" = __body.get_body_data_reader()?;\n");

    for input_field in &input_fields.fields {
        if !filter(input_field) {
            continue;
        }

        result.push_str("let ");
        result.push_str(input_field.struct_field_name());
        result.push_str(" = ");

        match &input_field.property.ty {
            PropertyType::FileContent => {
                result.push_str("FileContent{file_name: \"\".to_string(), content: Vec::new()};");
            }
            PropertyType::OptionOf(_) => {
                result.push_str("if let Some(value) = ");
                result.push_str(src_reader);
                result.push_str(".get_optional(\"");
                result.push_str(input_field.name());
                result.push_str("\"){");
                result.push_str("Some(value.try_into()?)}else{None};");
            }
            PropertyType::VecOf(_) => {}
            PropertyType::Struct(_) => {}
            _ => {
                generate_reading_required(result, input_field, src_reader);
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

    generate_init_fields(result, input_fields, &filter);
    result.push_str("};\n");
}

fn generate_reading_required(result: &mut String, input_field: &InputField, src_reader: &str) {
    match input_field.src {
        InputFieldSource::Query => {
            panic!("Bug. Query is not supported for read body model");
        }
        InputFieldSource::Path => {
            panic!("Bug. Path is not supported for read body model");
        }
        InputFieldSource::Header => {
            panic!("Bug. Path is not supported for read body model");
        }
        InputFieldSource::Body => {
            result.push_str(src_reader);
            result.push_str(".get_required(\"");
            result.push_str(input_field.name());
            result.push_str("\")?.try_into()?;");
        }
        InputFieldSource::FormData => {
            result.push_str(src_reader);
            result.push_str(".get_required(\"");
            result.push_str(input_field.name());
            result.push_str("\")?.try_into()?;");
        }
        InputFieldSource::BodyFile => {
            panic!("Bug. Should not read BodyFile at read body model");
        }
    }
}

fn generate_init_fields<TInputFiler: Fn(&InputField) -> bool>(
    result: &mut String,
    input_fields: &InputFields,
    filter: &TInputFiler,
) {
    let amount = input_fields.fields.iter().filter(|f| filter(f)).count();

    if amount > 1 {
        result.push('(');
    }

    let mut no = 0;
    for input_field in &input_fields.fields {
        if filter(input_field) {
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

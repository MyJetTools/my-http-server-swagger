use crate::{
    input_models::input_fields::{InputField, InputFields},
    reflection::PropertyType,
};

use super::consts::QUERY_STRING;

pub fn generate_reading_from_query_string(result: &mut String, input_fields: &InputFields) {
    let mut validation: Option<String> = None;

    result.push_str("let ");
    generate_init_fields(result, input_fields);

    result.push_str("={\n");

    result.push_str("let ");
    result.push_str(QUERY_STRING);

    result.push_str(" = ctx.request.get_query_string()?;\n");

    for input_field in &input_fields.fields {
        if !input_field.is_query_string() {
            continue;
        }

        result.push_str("let ");
        result.push_str(input_field.struct_field_name());
        result.push_str(": my_http_server::ValueAsString = ");

        match &input_field.property.ty {
            PropertyType::FileContent => {
                todo!("Not implemented yet");
            }
            PropertyType::OptionOf(sub_type) => {
                todo!("Not implemented yet");
            }
            PropertyType::VecOf(_) => {
                todo!("Not implemented yet");
            }
            PropertyType::Struct(_) => {
                todo!("Not implemented yet");
            }
            _ => {
                result.push_str(QUERY_STRING);
                result.push_str(".get_required(\"");
                result.push_str(input_field.name());
                result.push_str("\")?.into();");

                result.push_str("let ");
                result.push_str(input_field.struct_field_name());
                result.push_str(": ");
                result.push_str(input_field.property.ty.as_str().as_str());
                result.push_str(" = dt_from.try_into()?;");
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

fn generate_init_fields(result: &mut String, input_fields: &InputFields) {
    result.push('(');
    let mut no = 0;
    for input_field in &input_fields.fields {
        if input_field.is_query_string() {
            if no > 0 {
                result.push(',');
            }
            result.push_str(input_field.property.name.as_str());
            no += 1;
        }
    }

    result.push(')');
}
/*
fn generate_reading_required_value(result: &mut String, input_field: &InputField) {
    if let Some(default_value) = input_field.default() {
        if input_field.property.ty.is_string() {
            super::read_required_with_default::as_string(result, input_field, default_value);
            return;
        }

        if input_field.property.ty.is_boolean() {
            super::read_required_with_default::as_bool(result, input_field, default_value);
            return;
        }

        if input_field.property.ty.is_date_time() {
            super::read_required_with_default::as_date_time(result, input_field, default_value);
            return;
        }

        if input_field.property.ty.is_simple_type() {
            super::read_required_with_default::as_simple_type(
                result,
                input_field,
                &input_field.property.ty,
                default_value,
            );
            return;
        }

        super::read_required_with_default::parse_as_type(
            result,
            input_field,
            &input_field.property.ty,
            default_value,
        );
        return;
    }

    if let PropertyType::VecOf(sub_ty) = &input_field.property.ty {
        if sub_ty.is_string() {
            result.push_str(DATA_SOURCE);
            result.push_str(".get_vec_of_string(\"");
            result.push_str(input_field.name());
            result.push_str("\")?");
            return;
        }
        result.push_str(DATA_SOURCE);
        result.push_str(".get_vec(\"");
        result.push_str(input_field.name());
        result.push_str("\")?");
        return;
    }

    if input_field.property.ty.is_string() {
        super::read_required_value::as_string(result, input_field);
        return;
    }

    if input_field.property.ty.is_date_time() {
        super::read_required_value::as_date_time(result, input_field);
        return;
    }

    if input_field.property.ty.is_boolean() {
        super::read_required_value::as_bool(result, input_field);
        return;
    }

    super::read_required_value::parse_as_type(result, input_field);
}

fn generate_reading_optional_value(result: &mut String, input_field: &InputField) {
    if let PropertyType::OptionOf(generic_type) = &input_field.property.ty {
        if generic_type.is_string() {
            super::read_optional_value::as_string(result, input_field);
            result.push_str(";\n");
            return;
        } else if generic_type.is_boolean() {
            super::read_optional_value::as_bool(result, input_field);
            result.push_str(";\n");
            return;
        }
        super::read_optional_value::parase_as_type(result, input_field, generic_type);
        result.push_str(";\n");
        return;
    } else if let PropertyType::VecOf(_) = &input_field.property.ty {
        result.push_str("__data_source.get_vec(\"");
        result.push_str(input_field.name());
        result.push_str("\")?;");

        return;
    } else {
        panic!("Somehow we got here");
    }
}
 */

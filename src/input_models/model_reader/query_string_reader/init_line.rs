use crate::{
    input_models::input_fields::{InputField, InputFields},
    reflection::PropertyType,
};

use super::SourceToRead;

pub fn generate_init_line(result: &mut String, input_fields: &InputFields, src: SourceToRead) {
    result.push_str("let ");
    generate_init_fields(result, input_fields);

    result.push_str("={\n");

    result.push_str("let query_string = ctx.request.get_query_string()?;\n");

    for input_field in &input_fields.fields {
        if input_field.src.is_query() {
            result.push_str("let ");
            result.push_str(input_field.struct_field_name());
            result.push_str(" = ");

            if input_field.required() {
                generate_reading_required_value(result, input_field, &src);
                result.push_str(";\n");
            } else {
                generate_reading_optional_value(result, input_field, &src);
                result.push_str(";\n");
            }
        }
    }

    generate_init_fields(result, input_fields);
    result.push_str("};\n");
}

fn generate_init_fields(result: &mut String, input_fields: &InputFields) {
    result.push('(');
    let mut no = 0;
    for field in &input_fields.fields {
        if field.src.is_query() {
            if no > 0 {
                result.push(',');
            }
            result.push_str(field.property.name.as_str());
            no += 1;
        }
    }

    result.push(')');
}

fn generate_reading_required_value(
    result: &mut String,
    input_field: &InputField,
    src: &SourceToRead,
) {
    if let Some(default_value) = input_field.default() {
        if input_field.property.ty.is_string() {
            super::read_required_with_default::as_string(result, src, input_field, default_value);
            return;
        }

        if input_field.property.ty.is_boolean() {
            super::read_required_with_default::as_bool(result, src, input_field, default_value);
            return;
        }

        if input_field.property.ty.is_simple_type() {
            super::read_required_with_default::as_simple_type(
                result,
                src,
                input_field,
                default_value,
            );
            return;
        }

        super::read_required_with_default::parse_as_type(result, src, input_field, default_value);
        return;
    }

    if input_field.property.ty.is_string() {
        super::read_required_value::as_string(result, src, input_field);
        return;
    }

    if input_field.property.ty.is_boolean() {
        super::read_required_value::as_bool(result, src, input_field);
        return;
    }

    super::read_required_value::parse_as_type(result, src, input_field);
}

fn generate_reading_optional_value(
    result: &mut String,
    input_field: &InputField,
    src: &SourceToRead,
) {
    if let PropertyType::OptionOf(generic_type) = &input_field.property.ty {
        if generic_type.is_string() {
            super::read_optional_value::as_string(result, src, input_field);
            result.push_str(";\n");
            return;
        } else if generic_type.is_boolean() {
            super::read_optional_value::as_bool(result, src, input_field);
            result.push_str(";\n");
            return;
        }
        super::read_optional_value::parase_as_type(result, src, input_field);
        result.push_str(";\n");
        return;
    } else {
        panic!("Somehow we got here");
    }
}

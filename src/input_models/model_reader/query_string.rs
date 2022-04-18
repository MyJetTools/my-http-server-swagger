use crate::{
    input_models::input_fields::{InputField, InputFields},
    reflection::PropertyType,
};

use super::query_string_value_reader::SourceToRead;

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
                result.push_str(generate_reading_required_value(input_field, &src).as_str());
                result.push_str(";\n");
            } else {
                generate_reading_optional_value(result, input_field, &src);

                /*
                                if let PropertyType::OptionOf(inner_generic) = &input_field.property.ty {
                    if inner_generic.is_string() {
                        return super::rust_builders::read_optional_string_parameter(
                            source_to_read,
                            input_field,
                        );
                    } else if inner_generic.is_str() {
                        return super::rust_builders::read_optional_str_parameter(source_to_read, input_field);
                    } else {
                        return super::rust_builders::read_optional_parameter(source_to_read, input_field);
                    }
                }
                             */
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

fn generate_reading_required_value(input_field: &InputField, src: &SourceToRead) -> String {
    if let Some(default_value) = input_field.default() {
        if input_field.property.ty.is_string() {
            return super::query_string_value_reader::read_string_parameter_with_default_value(
                src,
                input_field,
                default_value,
            );
        }

        if input_field.property.ty.is_simple_type() {
            return super::query_string_value_reader::read_system_parameter_with_default_value(
                src,
                input_field,
                default_value,
            );
        }
        return super::query_string_value_reader::read_struct_parameter_with_default_value(
            src,
            input_field,
            default_value,
        );
    }

    if input_field.property.ty.is_string() {
        return super::query_string_value_reader::read_required_string_parameter(src, input_field);
    }

    if input_field.property.ty.is_simple_type() {
        return super::query_string_value_reader::read_required_simple_parameter(src, input_field);
    }

    return super::query_string_value_reader::read_required_struct_parameter(src, input_field);
}

fn generate_reading_optional_value(
    result: &mut String,
    input_field: &InputField,
    src: &SourceToRead,
) {
    if let PropertyType::OptionOf(generic_type) = &input_field.property.ty {
        if generic_type.is_string() {
            let line =
                super::query_string_value_reader::read_optional_string_parameter(src, input_field);
            result.push_str(line.as_str());
            result.push_str(",\n");
        } else {
            result.push_str(format!("{}", generic_type.as_str()).as_str());
        }
    } else {
        panic!("Somehow we got here");
    }
}

use crate::{input_models::input_fields::InputField, reflection::PropertyType};

pub fn generate_read_body(result: &mut String, input_field: &InputField) {
    result.push_str(input_field.property.name.as_str());

    if let PropertyType::VecOf(inner_generic) = &input_field.property.ty {
        if inner_generic.is_u8() {
            result.push_str(": ctx.request.receive_body().await?.get_body(),\n");
        } else {
            result.push_str(": ctx.request.get_body().await?.get_body_as_json()?,\n");
        }
    } else {
        result.push_str(": ctx.request.get_body().await?.get_body_as_json()?,\n");
    }
}

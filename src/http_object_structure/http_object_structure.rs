use crate::reflection::StructProperty;

use super::out_json::OutputJson;
use crate::consts::{HTTP_OBJECT_STRUCTURE, NAME_SPACE, USE_DOCUMENTATION};

pub fn generate(name: &str, fields: Vec<StructProperty>) -> String {
    let json = OutputJson::new(fields);

    let mut result = String::new();

    result.push_str(USE_DOCUMENTATION);

    result.push_str(format!("{NAME_SPACE}::{HTTP_OBJECT_STRUCTURE} {{").as_str());
    result.push_str(format!("struct_id: \"{}\".to_string(),", name).as_str());

    result.push_str("fields: vec![");

    for field in json.fields {
        crate::types::compile_http_field(
            &mut result,
            field.name(),
            &field.property.ty,
            true,
            None,
            None,
        );

        result.push(',');
    }
    result.push_str("],}");

    result
}

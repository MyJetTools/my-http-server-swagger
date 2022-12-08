use crate::consts::{HTTP_ENUM_STRUCTURE, HTTP_FAIL_RESULT, NAME_SPACE};

use proc_macro::TokenStream;

use crate::{
    enum_doc::enum_json::{EnumJson, HTTP_ENUM_ATTR_NAME},
    reflection::EnumCase,
};

pub fn generate(ast: &syn::DeriveInput, is_string: bool) -> TokenStream {
    let name = &ast.ident.to_string();
    let src_fields = EnumCase::read(ast);

    let mut fields = Vec::new();

    let mut default_case = None;

    for src_field in src_fields {
        let name = src_field.name.to_string();
        if let Some(enum_json) = EnumJson::new(src_field) {
            if enum_json.is_default_value {
                default_case = Some(enum_json.get_enum_case_value().to_string());
            }

            fields.push(enum_json);
        } else {
            panic!(
                "Enum case {} does not have #[{}] attribute",
                name, HTTP_ENUM_ATTR_NAME
            )
        }
    }

    let mut result = String::new();

    result.push_str("impl ");
    result.push_str(name);
    result.push('{');

    result.push_str("pub fn get_http_data_structure()->");
    result.push_str(NAME_SPACE);
    result.push_str("::");
    result.push_str(HTTP_ENUM_STRUCTURE);
    result.push('{');
    super::http_enum_structure::generate(&mut result, name.as_str(), is_string, fields.as_slice());
    result.push_str("}");

    result.push_str("fn create_default() -> Result<Self,");
    result.push_str(HTTP_FAIL_RESULT);
    result.push_str(">{");
    if let Some(default_case) = &default_case {
        result.push_str("Ok(Self::");
        result.push_str(default_case);
        result.push_str(")");
    } else {
        let line_to_add = format!(
            "Err({http_fail_result}::as_forbidden(Some(\"{err}\".to_string())))",
            http_fail_result = HTTP_FAIL_RESULT,
            err = format!("Type {} does not have default value to create", name)
        );
        result.push_str(line_to_add.as_str());
    }

    result.push_str("}");

    result.push('}');
    //Default Trait

    //FromStr Trait

    result.push_str("impl std::str::FromStr for ");
    result.push_str(name);
    result.push('{');
    super::impl_from_str_trait::generate(&mut result, name.as_str(), fields.as_slice());
    result.push('}');

    // From<number>

    if !is_string {
        result.push_str("impl From<i32> for ");
        result.push_str(name);
        result.push_str("{ fn from(src: i32) -> Self {");
        super::impl_from_i32::generate(&mut result, fields.as_slice());
        result.push_str("}}");
    }

    result.parse().unwrap()
}

use crate::consts::*;
use proc_macro::TokenStream;

use super::attributes::AttributeModel;

pub fn build_action(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut result = input.to_string();

    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let attrs = AttributeModel::parse(attr);

    let struct_name = ast.ident.to_string();

    result.push_str(
        format!(
            "impl {action_name} for {struct_name}{{",
            action_name = attrs.method.get_trait_name(),
        )
        .as_str(),
    );

    result.push_str("fn get_route(&self) -> &str {\"");
    result.push_str(attrs.route.as_str());
    result.push_str("\"}}\n");

    result.push_str("impl my_http_server_controllers::controllers::actions::GetDescription for ");

    result.push_str(struct_name.as_str());

    result.push('{');
    result.push_str(
        format!("fn get_description(&self) -> Option<{HTTP_ACTION_DESCRIPTION}>{{").as_str(),
    );
    super::generate_http_action_description_fn(&mut result, &attrs);
    result.push_str("}}");

    result.push_str("#[async_trait::async_trait]");

    result
        .push_str("impl my_http_server_controllers::controllers::actions::HandleHttpRequest for ");

    result.push_str(struct_name.as_str());

    result.push('{');

    result.push_str(
        format!("async fn handle_request(&self, http_route: &my_http_server_controllers::controllers::HttpRoute, ctx: &mut {HTTP_CONTEXT_WITH_SELF}) -> Result<{HTTP_OK_RESULT}, {HTTP_FAIL_RESULT}> {{")
            .as_str(),
    );
    super::generate_handle_request_fn(&mut result, &attrs.input_data);
    result.push_str("}\n");

    result.push_str("}");

    result.parse().unwrap()
}

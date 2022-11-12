use crate::consts::*;

use super::attributes::AttributeModel;

pub fn generate_http_action_description_fn(result: &mut String, attrs: &AttributeModel) {
    if attrs.api_data.is_none() {
        result.push_str("None");
        return;
    }

    let api_data = attrs.api_data.as_ref().unwrap();

    result.push_str(HTTP_ACTION_DESCRIPTION_WITH_NS);
    result.push_str("{");

    result.push_str("controller_name: \"");
    result.push_str(api_data.controller.as_str());
    result.push('"');
    result.push(',');

    result.push_str("summary: \"");
    result.push_str(api_data.summary.as_str());
    result.push('"');
    result.push(',');

    result.push_str("description: \"");
    result.push_str(api_data.description.as_str());
    result.push('"');
    result.push(',');

    result.push_str("should_be_authorized: ");
    result.push_str(api_data.should_be_authorized.as_str());
    result.push(',');

    result.push_str("input_params: ");
    generate_get_input_params(result, &attrs.input_data);
    result.push(',');

    result.push_str("results: ");
    super::result_model_generator::generate(result, &api_data.result);
    result.push_str("}.into()");
}

fn generate_get_input_params(result: &mut String, input_data: &Option<String>) {
    if let Some(input_data) = input_data {
        result.push_str(input_data);
        result.push_str("::get_input_params().into()");
    } else {
        result.push_str("None");
    }
}

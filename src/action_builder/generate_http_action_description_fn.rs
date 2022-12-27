use proc_macro2::TokenStream;

use super::attributes::AttributeModel;

pub fn generate_http_action_description_fn(attrs: &AttributeModel) -> TokenStream {
    if attrs.api_data.is_none() {
        return quote::quote!(None);
    }

    let api_data = attrs.api_data.as_ref().unwrap();

    let use_documentation = crate::consts::get_use_documentation();

    let http_action_description = crate::consts::get_http_action_description();

    let controller_name = api_data.controller.as_str();
    let summary = api_data.summary.as_str();
    let description = api_data.description.as_str();
    let should_be_authorized = api_data.get_should_be_authorized();

    let input_params = generate_get_input_params(&attrs.input_data);

    let results = super::result_model_generator::generate(&api_data.result);

    quote::quote! {
        #use_documentation;

        #http_action_description{
            controller_name: #controller_name,
            summary: #summary,
            description: #description,
            should_be_authorized: #should_be_authorized,
            input_params: #input_params,
            results: #results,
        }.into()

    }
}

fn generate_get_input_params(input_data: &Option<String>) -> TokenStream {
    if let Some(input_data) = input_data {
        let input_data = proc_macro2::Literal::string(input_data);
        quote::quote!(#input_data::get_input_params().into())
    } else {
        quote::quote!(in_parameters::HttpParameters::new(None))
    }
}

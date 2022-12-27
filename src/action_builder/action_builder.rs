use proc_macro::TokenStream;

use super::attributes::AttributeModel;

pub fn build_action(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let attrs = AttributeModel::parse(attr);

    let struct_name = &ast.ident;

    let trait_name = attrs.method.get_trait_name();

    let route = attrs.route.as_str();

    let http_action_description = crate::consts::get_http_action_description_with_ns();

    let description = super::generate_http_action_description_fn(&attrs);

    let http_route = crate::consts::get_http_route();

    let http_context = crate::consts::get_http_context();

    let http_ok_result = crate::consts::get_http_ok_result();

    let http_fail_result = crate::consts::get_http_fail_result();

    let handle_request = super::generate_handle_request_fn(&attrs.input_data);

    quote::quote! {

        impl #trait_name for #struct_name{
            fn get_route(&self) -> &str {
                #route
            }
        }

        impl my_http_server_controllers::controllers::actions::GetDescription for #struct_name{
            fn get_description(&self) -> Option<#http_action_description>{
                    #description
            }
        }

        #[async_trait::async_trait]
        impl my_http_server_controllers::controllers::actions::HandleHttpRequest for #struct_name{
            async fn handle_request(&self, http_route: &#http_route, ctx: &mut #http_context) -> Result<#http_ok_result, #http_fail_result> {
                    #handle_request
            }
        }
  
    }
    .into()
}

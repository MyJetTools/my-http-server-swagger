use proc_macro2::TokenStream;

pub fn generate_handle_request_fn(input_data: &Option<String>) -> TokenStream {
    if let Some(input_data) = input_data {
        let input_data = proc_macro2::Literal::string(input_data);
        quote::quote! {
            let input_data = #input_data::parse_http_input(http_route, ctx).await?;
            handle_request(self, input_data, ctx).await
        }
    } else {
        quote::quote!(handle_request(self, ctx).await)
    }
}

pub fn generate_handle_request_fn(result: &mut String, input_data: &Option<String>) {
    if let Some(input_data) = input_data {
        result.push_str(
            format!("let input_data = {input_data}::parse_http_input(ctx).await?;\n").as_str(),
        );
        result.push_str("handle_request(self, input_data, ctx).await");
    } else {
        result.push_str("handle_request(self, ctx).await");
    }
}

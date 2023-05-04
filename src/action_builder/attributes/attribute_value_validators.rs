use syn::DeriveInput;

pub fn validate_authorized_attribute_value(
    token_stream: &proc_macro::TokenStream,
    value: &str,
) -> Result<(), syn::Error> {
    //if !is_array_of_strings(value) {
    //    let ast: DeriveInput = syn::parse(token_stream.clone()).unwrap();
    //    return Err(syn::Error::new_spanned(ast, format!("The value of the attribute 'authorized' must be an array of strings. Example: ['admin', 'user'] er Empty array: Example: []. Or \"global\" or \"no\"")));
    // }

    Ok(())
}

fn is_array_of_strings(value: &str) -> bool {
    let value = value.trim();

    if !value.starts_with("[") {
        return false;
    }

    if !value.ends_with("]") {
        return false;
    }

    true
}

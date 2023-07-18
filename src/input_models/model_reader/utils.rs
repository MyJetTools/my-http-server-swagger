use proc_macro2::TokenStream;

use crate::input_models::InputField;

pub fn get_fields_to_read(fields: &[InputField]) -> Result<TokenStream, syn::Error> {
    if fields.len() == 1 {
        let name = fields.get(0).unwrap().property.get_field_name_ident();
        return Ok(quote::quote!(#name));
    }

    let mut no = 0;

    let mut result = Vec::with_capacity(fields.len());

    for input_field in fields {
        if no > 0 {
            result.push(quote::quote!(,));
        }

        let ident = input_field.property.get_field_name_ident();
        result.push(quote::quote!(#ident));
        no += 1;
    }

    Ok(quote::quote! {(#(#result)*)})
}

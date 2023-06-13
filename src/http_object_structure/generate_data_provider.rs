use crate::generic_utils::GenericData;

pub fn generate_data_provider(
    struct_name: &syn::Ident,
    generic_data: Option<&GenericData>,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let (generic, generic_ident, get_generic_type) = if let Some(generic) = generic_data {
        let generic_token_stream = generic.generic.clone();
        let generic_ident = generic.generic_ident.clone();

        let get_generic_type = quote::quote!(Some(#generic_ident));

        (generic_token_stream, generic_ident, get_generic_type)
    } else {
        (quote::quote! {}, quote::quote! {}, quote::quote!(None))
    };

    let result = quote::quote! {

        impl #generic my_http_server_controllers::controllers::documentation::DataTypeProvider for #struct_name #generic_ident {
            fn get_data_type() -> my_http_server_controllers::controllers::documentation::data_types::HttpDataType {
                Self::get_http_data_structure().into_http_data_type_object()
            }

            fn get_generic_type() -> Option<&'static str> {
               #get_generic_type
            }
        }
    };

    Ok(result)
}

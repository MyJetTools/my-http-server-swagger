use types_reader::StructProperty;

use crate::generic_utils::GenericData;

use super::struct_prop_ext::StructPropertyExt;

pub fn generate_data_structure_provider(
    ast: &syn::DeriveInput,
    struct_name: &syn::Ident,
    fields: &[StructProperty],
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let use_documentation = crate::consts::get_use_documentation();

    let (generic, generic_ident, generic_param) = if let Some(generic) = GenericData::new(ast) {
        let generic_token_stream = generic.generic;
        let generic_ident = generic.generic_ident;
        let mut generic_param = generic_ident.to_string();

        if generic_param.starts_with("<") {
            generic_param.remove(0);
            generic_param.remove(generic_param.len() - 1);
            generic_param = generic_param.trim().to_string();
        }
        (
            generic_token_stream,
            generic_ident,
            quote::quote! {Some(#generic_param)},
        )
    } else {
        (quote::quote! {}, quote::quote! {}, quote::quote! {None})
    };

    let struct_name_as_str = struct_name.to_string();

    let obj_fields = render_obj_fields(&fields)?;
    let result = quote::quote! {

        impl #generic my_http_server_controllers::controllers::documentation::DataTypeProvider for #struct_name #generic_ident {
            fn get_data_type() -> my_http_server_controllers::controllers::documentation::data_types::HttpDataType {
                #use_documentation;

                let mut __hos = data_types::HttpObjectStructure::new(#struct_name_as_str, #generic_param);
                #(#obj_fields)*
                __hos.into_http_data_type_object()
            }
        }
    };

    Ok(result)
}

fn render_obj_fields(
    fields: &[StructProperty],
) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut result = Vec::with_capacity(fields.len());
    for field in fields {
        let line = crate::types::compile_http_field(field.get_name()?, &field.ty, None)?;

        result.push(quote::quote!(__hos.fields.push(#line);));
    }

    Ok(result)
}

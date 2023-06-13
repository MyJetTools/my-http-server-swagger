use types_reader::StructProperty;

use crate::generic_utils::GenericData;

use super::struct_prop_ext::StructPropertyExt;

//TODO - Delete
pub fn generate_get_http_data_structure(
    struct_name: &syn::Ident,
    generic_data: Option<&GenericData>,
    fields: &[StructProperty],
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let use_documentation = crate::consts::get_use_documentation();

    let struct_name_as_str = struct_name.to_string();

    let fields = render_obj_fields(fields)?;

    let generic_name = if let Some(generic) = generic_data {
        let generic_param = generic.get_generic_name_as_string();
        quote:: quote!(Some(#generic_param))
    } else {
        quote::quote!(None)
    };

    let result = quote::quote! {
        pub fn get_http_data_structure()->my_http_server_controllers::controllers::documentation::data_types::HttpObjectStructure{
            #use_documentation;

            let mut _hos = data_types::HttpObjectStructure::new(#struct_name_as_str, #generic_name);
            #(#fields)*
            _hos
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

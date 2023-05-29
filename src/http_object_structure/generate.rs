use quote::quote;
use types_reader::StructProperty;

use crate::generic_utils::GenericData;

use super::struct_prop_ext::StructPropertyExt;

pub fn generate(ast: &syn::DeriveInput) -> (proc_macro::TokenStream, bool) {
    let struct_name = &ast.ident;

    let mut debug = false;

    let (generic, generic_ident) = if let Some(generic) = GenericData::new(ast) {
        let generic_token_stream = generic.generic;
        let generic_ident = generic.generic_ident;
        (generic_token_stream, generic_ident)
    } else {
        (quote! {}, quote! {})
    };

    let fields = match StructProperty::read(ast) {
        Ok(result) => result,
        Err(err) => return (err.into_compile_error().into(), debug),
    };

    for prop in &fields {
        if prop.attrs.has_attr("debug") {
            debug = true;
        }
    }

    let data_structure_provider =
        match super::generate_data_structure_provider(ast, struct_name, &fields) {
            Ok(result) => result,
            Err(err) => return (err.into_compile_error().into(), debug),
        };

    let fields = match generate_http_object_structure(fields) {
        Ok(result) => result,
        Err(err) => return (err.into_compile_error().into(), debug),
    };

    let use_documentation = crate::consts::get_use_documentation();

    let struct_name_as_str = struct_name.to_string();

    let result = quote! {
       #data_structure_provider

        impl #generic #struct_name #generic_ident {
            pub fn get_http_data_structure()->my_http_server_controllers::controllers::documentation::data_types::HttpObjectStructure{
                #use_documentation;

                data_types::HttpObjectStructure{
                    struct_id: #struct_name_as_str,
                    fields: vec![#(#fields),*]
                }
            }
        }

    }
    .into();

    (result, debug)
}

pub fn generate_http_object_structure(
    fields: Vec<StructProperty>,
) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut result = Vec::new();

    for field in fields {
        let line = crate::types::compile_http_field(field.get_name()?, &field.ty, None)?;

        result.push(line);
    }

    Ok(result)
}

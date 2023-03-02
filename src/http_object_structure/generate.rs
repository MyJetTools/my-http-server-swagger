use quote::quote;
use types_reader::StructProperty;

use crate::generic_utils::GenericData;

use super::struct_prop_ext::SturctPropertyExt;

pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let stuct_name = &ast.ident;

    let (generic, generic_ident) = if let Some(generic) = GenericData::new(ast) {
        let generic_token_stream = generic.generic;
        let generic_ident = generic.generic_ident;
        (
            Some(quote!(#generic_token_stream)),
            Some(quote!(#generic_ident)),
        )
    } else {
        (None, None)
    };

    let fields = match StructProperty::read(ast) {
        Ok(result) => result,
        Err(err) => return err.into_compile_error().into(),
    };

    let obj_fields = render_obj_fields(&fields);

    let fields = generate_http_object_structure(fields);

    let use_documentation = crate::consts::get_use_documentation();

    let struct_name_as_str = stuct_name.to_string();

    quote! {
        impl #generic #stuct_name #generic_ident {
            pub fn get_http_data_structure()->my_http_server_controllers::controllers::documentation::data_types::HttpObjectStructure{
                #use_documentation;

                data_types::HttpObjectStructure{
                    struct_id: #struct_name_as_str,
                    fields: vec![#(#fields),*]
                }
            }
        }

        impl #generic my_http_server_controllers::controllers::documentation::DataTypeProvider for #stuct_name #generic_ident {
            fn get_data_type() -> my_http_server_controllers::controllers::documentation::data_types::HttpDataType {
                #use_documentation;

                let mut __hos = data_types::HttpObjectStructure::new(#struct_name_as_str);
                #(#obj_fields)*
                __hos.into_http_data_type_object()
            }
        }
    }
    .into()
}

pub fn generate_http_object_structure(
    fields: Vec<StructProperty>,
) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::new();

    for field in fields {
        let line = crate::types::compile_http_field(field.get_name().as_str(), &field.ty, None);

        result.push(line);
    }

    result
}

fn render_obj_fields(fields: &[StructProperty]) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::with_capacity(fields.len());
    for field in fields {
        let line = crate::types::compile_http_field(field.get_name().as_str(), &field.ty, None);

        result.push(quote!(__hos.fields.push(#line);));
    }

    result
}

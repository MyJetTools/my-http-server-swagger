use quote::quote;
use types_reader::StructProperty;

pub fn impl_output_types(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let stuct_name = &ast.ident;
    let fields = StructProperty::read(ast);

    let fields = generate_http_object_structure(fields);

    let use_documentation = crate::consts::get_use_documentation();

    let struct_name_as_str = stuct_name.to_string();

    quote! {
        impl #stuct_name{
            pub fn get_http_data_structure()->my_http_server_controllers::controllers::documentation::data_types::HttpObjectStructure{
                #use_documentation;

                my_http_server_controllers::controllers::documentation::data_types::HttpObjectStructure{
                    struct_id: #struct_name_as_str.to_string(),
                    fields: vec![#(#fields)*,]
                }
            }
        }
    }
    .into()
}

pub fn generate_http_object_structure(
    fields: Vec<StructProperty>,
) -> Vec<proc_macro2::TokenStream> {
    let json = super::out_json::OutputJson::new(fields);

    let mut result = Vec::new();

    for field in json.fields {
        let line = crate::types::compile_http_field(
            field.name().get_value_as_str(),
            &field.property.ty,
            true,
            None,
        );

        result.push(line);
    }

    result
}

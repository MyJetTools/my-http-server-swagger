use proc_macro::TokenStream;

use crate::reflection::StructProperty;

use crate::consts::{HTTP_OBJECT_STRUCTURE, NAME_SPACE};

pub fn impl_output_types(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident.to_string();
    let fields = StructProperty::read(ast);

    let code = format!(
        r###" impl {name}{{
            pub fn {fn_name}()->{NAME_SPACE}::{HTTP_OBJECT_STRUCTURE}{{
                {get_http_object_structure}
            }}
        }}"###,
        get_http_object_structure = super::http_object_structure::generate(name.as_str(), fields),
        fn_name = crate::consts::FN_GET_HTTP_DATA_STRUCTURE
    );

    code.parse().unwrap()
}

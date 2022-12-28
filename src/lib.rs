extern crate proc_macro;
use proc_macro::TokenStream;

use syn;

mod action_builder;
mod as_token_stream;
mod consts;
mod enum_doc;
mod http_object_structure;
mod input_models;
mod proprety_type_ext;
mod types;

#[proc_macro_derive(
    MyHttpInput,
    attributes(
        http_query,
        http_header,
        http_path,
        http_form_data,
        http_body,
        http_body_type,
        http_body_file
    )
)]
pub fn my_http_input_doc_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::input_models::generate(&ast)
}

#[proc_macro_derive(MyHttpObjectStructure)]
pub fn my_http_input_process_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::http_object_structure::attr::impl_output_types(&ast)
}

#[proc_macro_derive(MyHttpStringEnum, attributes(http_enum_case))]
pub fn my_http_string_enum_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::enum_doc::generate(&ast, true)
}

#[proc_macro_derive(MyHttpIntegerEnum, attributes(http_enum_case))]
pub fn my_http_integer_enum_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::enum_doc::generate(&ast, false)
}

#[proc_macro_attribute]
pub fn http_route(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::action_builder::build_action(attr, item)
}

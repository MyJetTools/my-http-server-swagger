extern crate proc_macro;
use proc_macro::TokenStream;

use syn;

mod action_builder;
mod as_token_stream;
mod consts;
mod enum_doc;
mod generic_utils;
mod http_input_object_structure;
mod http_object_structure;
mod input_models;
mod property_type_ext;
mod types;

#[proc_macro_derive(
    MyHttpInput,
    attributes(
        http_query,
        http_header,
        http_path,
        http_form_data,
        http_body,
        http_body_raw,
    )
)]
pub fn my_http_input_doc_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let result = crate::input_models::generate(&ast);
    println!("{}", result);
    result
}

#[proc_macro_derive(MyHttpInputObjectStructure)]
pub fn my_http_input_object_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::http_input_object_structure::generate(&ast)
}

#[proc_macro_derive(MyHttpObjectStructure)]
pub fn my_http_output_object_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::http_object_structure::generate(&ast)
}

#[proc_macro_derive(MyHttpStringEnum, attributes(http_enum_case))]
pub fn my_http_string_enum_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::enum_doc::generate(&ast)
}

#[proc_macro_derive(MyHttpIntegerEnum, attributes(http_enum_case))]
pub fn my_http_integer_enum_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::enum_doc::generate(&ast)
}

#[proc_macro_attribute]
pub fn http_route(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::action_builder::build_action(attr, item)
}

use proc_macro2::TokenStream;

use super::enum_json::EnumJson;

pub fn generate(
    name: &str,
    is_string: bool,
    enum_cases: &[EnumJson],
) -> Result<TokenStream, syn::Error> {
    let mut cases = Vec::with_capacity(enum_cases.len());

    for enum_json in enum_cases {
        if let Some(data_to_add) = compile_enum_case(enum_json)? {
            cases.push(data_to_add);
        }
    }

    let use_documentation = crate::consts::get_use_documentation();

    let name_space = crate::consts::get_name_space();

    let http_enum_structure = crate::consts::get_http_enum_structure();

    let enum_type = crate::consts::get_enum_type();

    let tp = if is_string {
        quote::quote!(String)
    } else {
        quote::quote!(Integer)
    };

    let result = quote::quote! {
        #use_documentation;

        #name_space::#http_enum_structure{
            struct_id: #name,
            enum_type: #name_space::#enum_type::#tp,
            cases: vec![#(#cases),*],
        }
    };

    Ok(result)
}

fn compile_enum_case(enum_case: &EnumJson) -> Result<Option<TokenStream>, syn::Error> {
    let name_space = crate::consts::get_name_space();
    let http_enum_case = crate::consts::get_http_enum_case();

    let id = proc_macro2::Literal::isize_unsuffixed(enum_case.get_id()?);

    let value = enum_case.get_value();
    let value = value.get_value_as_str();

    let description = enum_case.description();
    let description = description.get_value_as_str();

    Ok(quote::quote! {
        #name_space::#http_enum_case{
            id: #id
            value: #value,
            description: #description
        }
    }
    .into())
}

use std::str::FromStr;

use proc_macro::TokenStream;
use types_reader::EnumCase;

use crate::enum_doc::enum_json::{EnumJson, HTTP_ENUM_ATTR_NAME};

pub fn generate(ast: &syn::DeriveInput, is_string: bool) -> TokenStream {
    let struct_name = &ast.ident;
    let struct_name_as_str = struct_name.to_string();

    let src_fields = EnumCase::read(ast);

    let mut fields = Vec::new();

    let mut default_case = None;

    for src_field in src_fields {
        let name = src_field.name.to_string();
        if let Some(enum_json) = EnumJson::new(src_field) {
            if enum_json.is_default_value {
                default_case = Some(enum_json.get_enum_case_value().to_string());
            }

            fields.push(enum_json);
        } else {
            panic!(
                "Enum case {} does not have #[{}] attribute",
                name, HTTP_ENUM_ATTR_NAME
            )
        }
    }

    //Default Trait

    let default_trait = if let Some(default_case) = &default_case {
        let default_case = proc_macro2::TokenStream::from_str(default_case).unwrap();

        let result = quote::quote! {
            impl std::default::Default for #struct_name{
                fn default() -> Self {
                    Self::#default_case
                }
            }
        };

        Some(result)
    } else {
        None
    };

    let name_space = crate::consts::get_name_space();
    let http_enum_structure = crate::consts::get_http_enum_structure();

    let http_fail_result = crate::consts::get_http_fail_result();

    let impl_get_http_data_structure = match super::http_enum_structure::generate(
        &struct_name_as_str,
        is_string,
        fields.as_slice(),
    ) {
        Ok(impl_get_http_data_structure) => impl_get_http_data_structure,
        Err(err) => err.to_compile_error(),
    };

    let create_default_impl = if default_case.is_some() {
        quote::quote!(Ok(Self::default()))
    } else {
        let err = format!(
            "Type {} does not have default value to create",
            struct_name_as_str
        );
        quote::quote! {
            Err(#http_fail_result::as_forbidden(Some(#err)))
        }
    };

    let impl_from_str =
        match super::impl_from_str_trait::generate(&struct_name_as_str.as_str(), fields.as_slice())
        {
            Ok(impl_from_str) => impl_from_str,
            Err(err) => vec![err.to_compile_error()],
        };

    let impl_from_i32 = match super::impl_from_i32::generate(fields.as_slice()) {
        Ok(impl_from_i32) => impl_from_i32,
        Err(err) => vec![err.to_compile_error()],
    };

    quote::quote! {
        impl #struct_name{

            pub fn get_http_data_structure()->#name_space::#http_enum_structure{
                #impl_get_http_data_structure
            }

            pub fn create_default() -> Result<Self,#http_fail_result>{
                #create_default_impl.to_string()
            }

        }

        #default_trait

        impl std::str::FromStr for #struct_name{
            type Err = #http_fail_result;
            fn from_str(src:&str)->Result<Self,Self::Err>{
                #(#impl_from_str)*
            }
        }

        impl From<i32> for #struct_name{
            fn from(src: i32) -> Self {
                match src {
                #(#impl_from_i32)*
                }
            }
        }

    }
    .into()
}

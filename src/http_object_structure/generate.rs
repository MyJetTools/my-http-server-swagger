use std::str::FromStr;

use quote::{quote, ToTokens};
use types_reader::StructProperty;

use super::struct_prop_ext::SturctPropertyExt;

pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let stuct_name = &ast.ident;
    let generic = &ast.generics;



    let (generic, generic_ident, generic_with_life_time) = if generic.params.is_empty() {
       (None, None, Some(quote!(<'s>)))
    } else {
        let generic_ident = generic.params.to_token_stream().to_string();
        let generic_ident_pos = generic_ident.find(':').unwrap();

        let gen = &generic_ident.as_bytes()[..generic_ident_pos];
        let gen = std::str::from_utf8(gen).unwrap();
        println!("generic_ident: {}", gen);



        let generic_ident = proc_macro2::TokenStream::from_str(gen).unwrap();

        let generic_with_lifetime = generic.to_token_stream().to_string();

        let as_bytes = &generic_with_lifetime.as_bytes()[1..generic_with_lifetime.len()-1];

        let generics_with_lifetime = proc_macro2::TokenStream::from_str(std::str::from_utf8(as_bytes).unwrap()).unwrap();

        
        (Some(quote!(#generic)),   Some(quote!(<#generic_ident>)), Some(quote!(<'s, #generics_with_lifetime>)))
    };

    let fields = match StructProperty::read(ast){
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

        impl #generic_with_life_time TryFrom<my_http_server::InputParamValue<'s>> for #stuct_name #generic_ident {
            type Error = my_http_server::HttpFailResult;
        
            fn try_from(value: my_http_server::InputParamValue) -> Result<Self, Self::Error> {
                value.from_json()
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

        impl #generic TryFrom<my_http_server::HttpRequestBody> for #stuct_name #generic_ident {
            type Error = my_http_server::HttpFailResult;
        
            fn try_from(value: my_http_server::HttpRequestBody) -> Result<Self, Self::Error> {
                value.get_body_as_json()
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
        let line = crate::types::compile_http_field(
            field.get_name().as_str(),
            &field.ty,
            None,
        );

        result.push(line);
    }

    result
}


fn render_obj_fields(fields: &[StructProperty])->Vec<proc_macro2::TokenStream>{

    let mut result = Vec::with_capacity(fields.len());
    for field in fields {
        let line = crate::types::compile_http_field(
            field.get_name().as_str(),
            &field.ty,
            None,
        );

        result.push(quote!(__hos.fields.push(#line);));
    }

    result


}
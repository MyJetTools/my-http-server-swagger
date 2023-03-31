use quote::quote;
use types_reader::StructProperty;

use crate::generic_utils::GenericData;

use super::struct_prop_ext::StructPropertyExt;

pub fn generate(ast: &syn::DeriveInput) -> (proc_macro::TokenStream, bool) {
    let struct_name = &ast.ident;

    let mut debug = false;

    let (generic, generic_no_brackets,  generic_ident, generic_ident_no_brackets) = if let Some(generic) = GenericData::new(ast) {
        let generic_no_brackets = generic.get_generic_no_brackets();
        let generic_ident_no_brackets = generic.get_generic_no_brackets();
        let generic_token_stream = generic.generic;
        let generic_ident = generic.generic_ident;
        (
            generic_token_stream,
            generic_no_brackets,
            generic_ident,
            generic_ident_no_brackets,
        )
    } else {
        (quote!{}, quote!{}, quote!{}, quote!{})
    };

    let fields = match StructProperty::read(ast) {
        Ok(result) => result,
        Err(err) => return (err.into_compile_error().into(), debug),
    };

    for prop in &fields{
        if prop.attrs.has_attr("debug"){
            debug = true;
        }
    }

    let obj_fields = render_obj_fields(&fields);

    let fields = generate_http_object_structure(fields);

    let use_documentation = crate::consts::get_use_documentation();

    let struct_name_as_str = struct_name.to_string();

   let result = quote! {
        impl #generic #struct_name #generic_ident {
            pub fn get_http_data_structure()->my_http_server_controllers::controllers::documentation::data_types::HttpObjectStructure{
                #use_documentation;

                data_types::HttpObjectStructure{
                    struct_id: #struct_name_as_str,
                    fields: vec![#(#fields),*]
                }
            }
        }

        impl #generic my_http_server_controllers::controllers::documentation::DataTypeProvider for #struct_name #generic_ident {
            fn get_data_type() -> my_http_server_controllers::controllers::documentation::data_types::HttpDataType {
                #use_documentation;

                let mut __hos = data_types::HttpObjectStructure::new(#struct_name_as_str);
                #(#obj_fields)*
                __hos.into_http_data_type_object()
            }
        }

        impl<'s, #generic_no_brackets> TryInto<#struct_name #generic_ident> for my_http_server::InputParamValue<'s> {
            type Error = my_http_server::HttpFailResult;
        
            fn try_into(self) -> Result<#struct_name #generic_ident, Self::Error> {
                self.from_json()
            }
        }


  
    }
    .into();

  (result, debug)
}

/*

        impl #generic TryInto<#generic #generic_ident> for my_http_server::HttpRequestBody {
            type Error = my_http_server::HttpFailResult;
            fn try_into(self) -> Result<#struct_name, my_http_server::HttpFailResult> {
                self.get_body_as_json()
            }
        }
 */

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

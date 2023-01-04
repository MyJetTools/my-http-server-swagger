use proc_macro2::TokenStream;
use quote::quote;
use types_reader::attribute_params::ParamValue;

use crate::input_models::input_fields::InputField;
pub trait AsTokenStream {
    fn as_token_stream(&self) -> TokenStream;
}

impl<'s> AsTokenStream for Option<ParamValue<'s>> {
    fn as_token_stream(&self) -> TokenStream {
        if let Some(value) = self {
            let value = value.as_str();
            quote! {
                Some(#value)
            }
        } else {
            quote! {
                None
            }
        }
    }
}

impl<'s> AsTokenStream for Vec<&'s InputField<'s>> {
    fn as_token_stream(&self) -> TokenStream {
        if self.len() == 1 {
            let name = self.get(0).unwrap().property.get_field_name_ident();
            return quote!(#name);
        }

        let mut no = 0;

        let mut result = Vec::with_capacity(self.len());

        for input_field in self {
            if no > 0 {
                result.push(quote!(,));
            }

            let ident = input_field.property.get_field_name_ident();
            result.push(quote!(#ident));
            no += 1;
        }

        quote! {(#(#result)*)}
    }
}

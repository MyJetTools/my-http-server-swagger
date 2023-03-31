use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::ToTokens;

pub struct GenericData {
    pub generic: TokenStream,
    pub generic_ident: TokenStream,
}

impl GenericData {
    pub fn new(ast: &syn::DeriveInput) -> Option<Self> {
        if ast.generics.params.is_empty() {
            return None;
        }

        let generics = &ast.generics;

        let generic_ident = generics.params.to_token_stream().to_string();
        let generic_ident_pos = generic_ident.find(':').unwrap();

        let gen = &generic_ident.as_bytes()[..generic_ident_pos];
        let gen = std::str::from_utf8(gen).unwrap();

        let generic_ident = proc_macro2::TokenStream::from_str(gen).unwrap();

        Self {
            generic: quote::quote!(#generics),
            generic_ident: quote::quote!(<#generic_ident>),
        }
        .into()
    }

    pub fn get_generic_no_brackets(&self) -> TokenStream {
        let generic = self.generic.to_string();
        let generic = generic.replace("<", "");
        let generic = generic.replace(">", "");
        generic.parse().unwrap()
    }
}

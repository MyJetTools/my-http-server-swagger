use super::HttpResult;

pub struct ApiData<'s> {
    pub controller: &'s str,
    pub description: &'s str,
    pub summary: &'s str,
    pub should_be_authorized: Option<&'s Vec<String>>,
    pub results: Option<Vec<HttpResult>>,
}

impl<'s> ApiData<'s> {
    pub fn new(
        controller: &'s str,
        attrs: &'s types_reader::ParamsList,
    ) -> Result<Self, syn::Error> {
        let description = attrs.get_named_param("description")?.get_str_value()?;
        let summary = attrs.get_named_param("summary")?.get_str_value()?;
        let should_be_authorized =
            if let Some(value) = attrs.try_get_named_param("should_be_authorized") {
                Some(value.unwrap_as_vec_of_string()?)
            } else {
                None
            };

        let results = if let Some(result) = attrs.try_get_named_param("result") {
            Some(result.unwrap_as_object_list()?)
        } else {
            None
        };

        let results = if let Some(http_results) = results {
            let mut result = Vec::new();

            for param_list in http_results {
                result.push(HttpResult::new(param_list)?);
            }

            Some(result)
        } else {
            None
        };

        Ok(Self {
            controller,
            description,
            summary,
            results,
            should_be_authorized,
        })
    }

    pub fn get_should_be_authorized(&self) -> proc_macro2::TokenStream {
        if self.should_be_authorized.is_none() {
            return quote::quote!(ShouldBeAuthorized::UseGlobal);
        }

        let should_be_authorized = self.should_be_authorized.unwrap();

        if should_be_authorized.is_empty() {
            return quote::quote!(ShouldBeAuthorized::YesWithClaims(
                RequiredClaims::no_claims()
            ));
        }

        let mut result = Vec::new();

        if let Some(should_be_authorized) = self.should_be_authorized {
            for itm in should_be_authorized {
                result.push(quote::quote!(#itm.to_string()));
            }
        }

        quote::quote!(ShouldBeAuthorized::YesWithClaims(RequiredClaims::from_vec(
            vec![#(#result)*,]
        )))
        .into()
    }
}

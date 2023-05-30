use types_reader::ParamValue;

use super::{http_method::HttpMethod, ApiData};

pub struct HttpRouteModel<'s> {
    pub method: HttpMethod,
    pub route: &'s str,
    pub input_data: Option<&'s str>,
    pub api_data: Option<ApiData<'s>>,
    pub should_be_authorized: Option<&'s ParamValue>,
}

impl<'s> HttpRouteModel<'s> {
    pub fn parse(attrs: &'s types_reader::ParamsList) -> Result<Self, syn::Error> {
        let method = attrs
            .get_named_param("method")?
            .unwrap_as_string_value()?
            .as_str();

        let route = attrs
            .get_named_param("route")?
            .unwrap_as_string_value()?
            .as_str();

        let input_data = if let Some(input_data) = attrs.try_get_named_param("input_data") {
            Some(input_data.unwrap_as_string_value()?.as_str())
        } else {
            None
        };

        let should_be_authorized = attrs.try_get_named_param("authorized");

        let result = if let Some(controller) = attrs.try_get_named_param("controller") {
            let controller = controller.unwrap_as_string_value()?.as_str();

            Ok(Self {
                method: HttpMethod::parse(method),
                route,
                input_data,
                should_be_authorized,
                api_data: Some(ApiData::new(controller, attrs)?),
            })
        } else {
            Ok(Self {
                method: HttpMethod::parse(method),
                route,
                input_data,
                should_be_authorized,
                api_data: None,
            })
        };

        result

        /*
        let attr = attr_src.to_string();

        let str = attr.into_bytes();

        let mut bytes = str.as_slice();

        let mut method: Option<String> = None;
        let mut route: Option<String> = None;

        let mut controller: Option<String> = None;
        let mut description: Option<String> = None;

        let mut summary: Option<String> = None;
        let mut input_data: Option<String> = None;
        let mut result: Option<String> = None;
        let mut should_be_authorized: Option<String> = None;

        loop {
            let separator_pos = find(bytes, ':' as u8);

            if separator_pos.is_none() {
                break;
            }

            let separator_pos = separator_pos.unwrap();

            let key = std::str::from_utf8(&bytes[..separator_pos]).unwrap().trim();

            bytes = &bytes[separator_pos..];

            let start_value_pos = find_one_of_these(bytes, &['[' as u8, '"' as u8]);

            if start_value_pos.is_none() {
                break;
            }

            let start_value_pos = start_value_pos.unwrap();

            bytes = &bytes[start_value_pos..];

            let end_byte = if bytes[0] == '[' as u8 {
                ']' as u8
            } else {
                bytes[0]
            };

            bytes = &bytes[1..];

            let end_value_pos = find(bytes, end_byte);

            if end_value_pos.is_none() {
                break;
            }

            let end_value_pos = end_value_pos.unwrap();

            let value = std::str::from_utf8(&bytes[..end_value_pos]).unwrap();

            match key {
                "method" => {
                    method = Some(value.to_string());
                }
                "controller" => {
                    controller = Some(value.to_string());
                }
                "route" => {
                    route = Some(value.to_string());
                }
                "description" => {
                    description = Some(value.to_string());
                }
                "summary" => {
                    summary = Some(value.to_string());
                }
                "input_data" => {
                    input_data = Some(value.to_string());
                }

                "result" => {
                    result = Some(value.to_string());
                }

                "authorized" => match value {
                    "[]" => {
                        should_be_authorized = Some("ShouldBeAuthorized::Yes".to_string());
                    }

                    "global" => {
                        should_be_authorized = Some("ShouldBeAuthorized::UseGlobal".to_string());
                    }
                    "no" => {
                        should_be_authorized = Some("ShouldBeAuthorized::No".to_string());
                    }
                    _ => {
                        super::validate_authorized_attribute_value(&attr_src, value)?;
                        should_be_authorized =
                        Some(format!("ShouldBeAuthorized::YesWithClaims(my_http_server_controllers::controllers::RequiredClaims::from_slice_of_str(&[{}]))", value));
                    }
                },

                _ => {}
            }

            bytes = &bytes[end_value_pos..];

            let separator_pos = find(bytes, ',' as u8);

            if separator_pos.is_none() {
                break;
            }

            let separator_pos = separator_pos.unwrap();
            bytes = &bytes[separator_pos + 1..];
        }

        if method.is_none() {
            panic!("[method] is not found");
        }

        if route.is_none() {
            panic!("[route] is not found");
        }

        if should_be_authorized.is_none() {
            should_be_authorized = Some("ShouldBeAuthorized::UseGlobal".to_string());
        }

        Ok(Self {
            method: HttpMethod::parse(method.as_ref().unwrap()),
            route: route.unwrap(),
            input_data,
            api_data: ApiData::new(
                controller,
                description,
                summary,
                should_be_authorized.unwrap(),
                HttpResult::new(result),
            ),
        })
         */
    }

    pub fn get_should_be_authorized(&self) -> Result<proc_macro2::TokenStream, syn::Error> {
        if self.should_be_authorized.is_none() {
            return Ok(quote::quote!(ShouldBeAuthorized::UseGlobal));
        }

        let should_be_authorized = self.should_be_authorized.unwrap();

        if let Some(string_value) = should_be_authorized.try_unwrap_as_string_value() {
            let value = string_value.as_str();

            if value == "Yes" {
                return Ok(quote::quote!(ShouldBeAuthorized::Yes));
            }

            if value == "No" {
                return Ok(quote::quote!(ShouldBeAuthorized::No));
            }

            return Err(should_be_authorized
                .throw_error("Unsupported value. It should be Yes, No or Array of strings"));
        }

        if let Some(string_value) = should_be_authorized.try_unwrap_as_vec_of_string() {
            let mut result = Vec::new();

            for itm in string_value {
                result.push(quote::quote!(#itm.to_string()));
            }

            return Ok(
                quote::quote!(ShouldBeAuthorized::YesWithClaims(RequiredClaims::from_vec(
                    vec![#(#result)*,]
                )))
                .into(),
            );
        }

        Err(should_be_authorized.throw_error("Unsupported data type"))
    }
}

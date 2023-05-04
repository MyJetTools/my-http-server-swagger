use std::str::FromStr;

use proc_macro2::TokenStream;

use super::{http_method::HttpMethod, HttpResult};

pub struct ApiData {
    pub controller: String,
    pub description: String,
    pub summary: String,
    pub should_be_authorized: String,
    pub result: Vec<HttpResult>,
}

impl ApiData {
    pub fn new(
        controller: Option<String>,
        description: Option<String>,
        summary: Option<String>,
        should_be_authorized: String,
        result: Vec<HttpResult>,
    ) -> Option<Self> {
        if controller.is_none() {
            return None;
        }

        if description.is_none() {
            panic!("Description is not found");
        }

        if summary.is_none() {
            panic!("Summary is not found");
        }

        Self {
            controller: controller.unwrap(),
            description: description.unwrap(),
            summary: summary.unwrap(),
            result,
            should_be_authorized,
        }
        .into()
    }

    pub fn get_should_be_authorized(&self) -> TokenStream {
        TokenStream::from_str(&self.should_be_authorized).unwrap()
    }
}

pub struct AttributeModel {
    pub method: HttpMethod,
    pub route: String,
    pub input_data: Option<String>,
    pub api_data: Option<ApiData>,
}

impl AttributeModel {
    pub fn parse(attr_src: proc_macro::TokenStream) -> Result<Self, syn::Error> {
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
    }
}

pub fn find(src: &[u8], symbol: u8) -> Option<usize> {
    for i in 0..src.len() {
        if src[i] == symbol {
            return Some(i);
        }
    }

    None
}

pub fn find_one_of_these(src: &[u8], symbols: &[u8]) -> Option<usize> {
    for i in 0..src.len() {
        for s in symbols {
            if src[i] == *s {
                return Some(i);
            }
        }
    }

    None
}

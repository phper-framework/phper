use crate::{errors::HttpClientError, utils::replace_and_get};
use indexmap::map::IndexMap;
use phper::{
    classes::{DynamicClass, Visibility},
    objects::Object,
};
use reqwest::blocking::Response;

pub const RESPONSE_CLASS_NAME: &'static str = "HttpClient\\Response";

pub fn make_response_class() -> DynamicClass<Option<Response>> {
    let mut class = DynamicClass::new_with_none(RESPONSE_CLASS_NAME);

    class.add_method(
        "body",
        Visibility::Public,
        |this: &mut Object<Option<Response>>, _arguments| {
            let response = this.as_mut_state();
            let body = replace_and_get(response, None, |response| {
                response
                    .ok_or(HttpClientError::ResponseHadRead)
                    .and_then(|response| response.bytes().map_err(Into::into))
            })?;
            Ok::<_, HttpClientError>((&body).to_vec())
        },
        vec![],
    );

    class.add_method(
        "status",
        Visibility::Public,
        |this, _arguments| {
            let response =
                this.as_state()
                    .as_ref()
                    .ok_or_else(|| HttpClientError::ResponseAfterRead {
                        method_name: "status".to_owned(),
                    })?;
            Ok::<_, HttpClientError>(response.status().as_u16() as i64)
        },
        vec![],
    );

    class.add_method(
        "headers",
        Visibility::Public,
        |this, _arguments| {
            let response =
                this.as_state()
                    .as_ref()
                    .ok_or_else(|| HttpClientError::ResponseAfterRead {
                        method_name: "headers".to_owned(),
                    })?;
            let headers_map =
                response
                    .headers()
                    .iter()
                    .fold(IndexMap::new(), |mut acc, (key, value)| {
                        acc.entry(key.as_str().to_owned())
                            .or_insert(vec![])
                            .push(value.as_bytes().to_owned());
                        acc
                    });
            Ok::<_, HttpClientError>(headers_map)
        },
        vec![],
    );

    class
}

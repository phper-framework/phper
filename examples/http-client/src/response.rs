// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::{errors::HttpClientError, utils::replace_and_get};
use phper::{
    arrays::{InsertKey, ZArray},
    classes::{DynamicClass, Visibility},
    objects::ZObj,
    values::ZVal,
};
use reqwest::blocking::Response;

pub const RESPONSE_CLASS_NAME: &str = "HttpClient\\Response";

pub fn make_response_class() -> DynamicClass<Option<Response>> {
    let mut class = DynamicClass::new_with_default(RESPONSE_CLASS_NAME);

    class.add_method(
        "body",
        Visibility::Public,
        |this: &mut ZObj<Option<Response>>, _arguments| {
            let response = unsafe { this.as_mut_state() };
            let body = replace_and_get(response, |response| {
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
            let response = unsafe { this.as_state() }.as_ref().ok_or_else(|| {
                HttpClientError::ResponseAfterRead {
                    method_name: "status".to_owned(),
                }
            })?;

            Ok::<_, HttpClientError>(response.status().as_u16() as i64)
        },
        vec![],
    );

    class.add_method(
        "headers",
        Visibility::Public,
        |this, _arguments| {
            let response = unsafe {
                this.as_state()
                    .as_ref()
                    .ok_or_else(|| HttpClientError::ResponseAfterRead {
                        method_name: "headers".to_owned(),
                    })?
            };
            let headers_map =
                response
                    .headers()
                    .iter()
                    .fold(ZArray::new(), |mut acc, (key, value)| {
                        let arr = acc.entry(key.as_str()).or_insert(ZVal::from(ZArray::new()));
                        arr.as_mut_z_arr()
                            .unwrap()
                            .insert(InsertKey::NextIndex, ZVal::from(value.as_bytes()));
                        acc
                    });
            Ok::<_, HttpClientError>(headers_map)
        },
        vec![],
    );

    class
}

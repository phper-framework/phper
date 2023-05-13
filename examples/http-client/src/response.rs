// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::errors::HttpClientError;
use phper::{
    arrays::{InsertKey, ZArray},
    classes::{ClassEntity, StaticStateClass, Visibility},
    values::ZVal,
};
use reqwest::blocking::Response;
use std::mem::take;

pub const RESPONSE_CLASS_NAME: &str = "HttpClient\\Response";

pub static RESPONSE_CLASS: StaticStateClass<Option<Response>> = StaticStateClass::null();

pub fn make_response_class() -> ClassEntity<Option<Response>> {
    let mut class =
        ClassEntity::<Option<Response>>::new_with_default_state_constructor(RESPONSE_CLASS_NAME);

    class.bind(&RESPONSE_CLASS);

    class.add_method("body", Visibility::Public, |this, _arguments| {
        let response = take(this.as_mut_state());
        let response = response.ok_or(HttpClientError::ResponseHadRead)?;
        let body = response.bytes().map_err(HttpClientError::Reqwest)?;
        Ok::<_, phper::Error>(body.to_vec())
    });

    class.add_method("status", Visibility::Public, |this, _arguments| {
        let response =
            this.as_state()
                .as_ref()
                .ok_or_else(|| HttpClientError::ResponseAfterRead {
                    method_name: "status".to_owned(),
                })?;

        Ok::<_, HttpClientError>(response.status().as_u16() as i64)
    });

    class.add_method("headers", Visibility::Public, |this, _arguments| {
        let response =
            this.as_state()
                .as_ref()
                .ok_or_else(|| HttpClientError::ResponseAfterRead {
                    method_name: "headers".to_owned(),
                })?;
        let headers_map = response
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
    });

    class
}

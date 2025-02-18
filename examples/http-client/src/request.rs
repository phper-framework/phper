// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::{errors::HttpClientError, response::ResponseClass};
use phper::classes::{ClassEntity, StateClass, Visibility};
use reqwest::blocking::RequestBuilder;
use std::{convert::Infallible, mem::take};

pub type RequestBuilderClass = StateClass<Option<RequestBuilder>>;

pub const REQUEST_BUILDER_CLASS_NAME: &str = "HttpClient\\RequestBuilder";

pub fn make_request_builder_class(
    response_class: ResponseClass,
) -> ClassEntity<Option<RequestBuilder>> {
    let mut class = ClassEntity::<Option<RequestBuilder>>::new_with_default_state_constructor(
        REQUEST_BUILDER_CLASS_NAME,
    );

    class.add_method("__construct", Visibility::Private, |_, _| {
        Ok::<_, Infallible>(())
    });

    class.add_method("send", Visibility::Public, move |this, _arguments| {
        let state = take(this.as_mut_state());
        let response = state.unwrap().send().map_err(HttpClientError::Reqwest)?;
        let mut object = response_class.new_object([])?;
        *object.as_mut_state() = Some(response);
        Ok::<_, phper::Error>(object)
    });

    class
}

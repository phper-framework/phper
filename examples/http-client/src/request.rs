// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::{errors::HttpClientError, response::RESPONSE_CLASS_NAME, utils::replace_and_get};
use phper::{
    classes::{ClassEntry, DynamicClass, Visibility},
    objects::ZObj,
};
use reqwest::blocking::{RequestBuilder, Response};

pub const REQUEST_BUILDER_CLASS_NAME: &str = "HttpClient\\RequestBuilder";

pub fn make_request_builder_class() -> DynamicClass<Option<RequestBuilder>> {
    let mut class = DynamicClass::new_with_default(REQUEST_BUILDER_CLASS_NAME);

    class.add_method(
        "__construct",
        Visibility::Private,
        |_: &mut ZObj, _| {},
        vec![],
    );

    class.add_method(
        "send",
        Visibility::Public,
        |this, _arguments| {
            let state = unsafe { this.as_mut_state::<Option<RequestBuilder>>() };
            let response = replace_and_get(state, |builder| builder.unwrap().send())?;
            let mut object = ClassEntry::from_globals(RESPONSE_CLASS_NAME)?.new_object([])?;
            unsafe {
                *object.as_mut_state() = Some(response);
            }
            Ok::<_, HttpClientError>(object)
        },
        vec![],
    );

    class
}

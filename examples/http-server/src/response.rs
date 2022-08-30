// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::errors::HttpServerError;
use hyper::{header::HeaderName, http::HeaderValue, Body, Response};
use phper::{
    classes::{StatefulClass, Visibility},
    functions::Argument,
};

pub const HTTP_RESPONSE_CLASS_NAME: &str = "HttpServer\\HttpResponse";

pub fn make_response_class() -> StatefulClass<Response<Body>> {
    let mut class = StatefulClass::new_with_default_state(HTTP_RESPONSE_CLASS_NAME);

    class.add_method(
        "header",
        Visibility::Public,
        |this, arguments| {
            let response: &mut Response<Body> = this.as_mut_state();
            response.headers_mut().insert(
                HeaderName::from_bytes(arguments[0].as_z_str().unwrap().to_bytes())?,
                HeaderValue::from_bytes(arguments[1].as_z_str().unwrap().to_bytes())?,
            );
            Ok::<_, HttpServerError>(())
        },
        vec![Argument::by_val("data")],
    );

    class.add_method(
        "end",
        Visibility::Public,
        |this, arguments| {
            let response: &mut Response<Body> = this.as_mut_state();
            *response.body_mut() = arguments[0].as_z_str().unwrap().to_bytes().to_vec().into();
            Ok::<_, phper::Error>(())
        },
        vec![Argument::by_val("data")],
    );

    class
}

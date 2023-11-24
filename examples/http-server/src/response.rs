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
use axum::{
    body::Body,
    http::{HeaderName, HeaderValue, Response},
};
use phper::{
    classes::{ClassEntity, StaticStateClass, Visibility},
    functions::Argument,
    objects::StateObject,
};

pub const HTTP_RESPONSE_CLASS_NAME: &str = "HttpServer\\HttpResponse";

pub static HTTP_RESPONSE_CLASS: StaticStateClass<Response<Body>> = StaticStateClass::null();

/// Register the class `HttpServer\HttpResponse` by `ClassEntity`, with the
/// inner state `Response<Body>`.
pub fn make_response_class() -> ClassEntity<Response<Body>> {
    let mut class = ClassEntity::new_with_default_state_constructor(HTTP_RESPONSE_CLASS_NAME);

    // The state class will be initialized after class registered.
    class.bind(&HTTP_RESPONSE_CLASS);

    // Register the header method with public visibility, accept `name` and `value`
    // parameters.
    class
        .add_method("header", Visibility::Public, |this, arguments| {
            let name = arguments[0].expect_z_str()?.to_bytes();
            let value = arguments[1].expect_z_str()?.to_bytes();

            // Inject the header into inner response state.
            let response: &mut Response<Body> = this.as_mut_state();
            response.headers_mut().insert(
                HeaderName::from_bytes(name).map_err(HttpServerError::new)?,
                HeaderValue::from_bytes(value).map_err(HttpServerError::new)?,
            );

            Ok::<_, phper::Error>(())
        })
        .argument(Argument::by_val("name"))
        .argument(Argument::by_val("value"));

    // Register the end method with public visibility, accept `data` parameters.
    class
        .add_method("end", Visibility::Public, |this, arguments| {
            // Inject the body content into inner response state.
            let response: &mut Response<Body> = this.as_mut_state();
            *response.body_mut() = arguments[0].expect_z_str()?.to_bytes().to_vec().into();
            Ok::<_, phper::Error>(())
        })
        .argument(Argument::by_val("data"));

    class
}

/// Instantiate the object with class `HttpServer\HttpResponse`.
pub fn new_response_object() -> phper::Result<StateObject<Response<Body>>> {
    HTTP_RESPONSE_CLASS.new_object([])
}

// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{
    arrays::ZArray,
    classes::{ClassEntity, StaticStateClass, Visibility},
    objects::StateObject,
};
use std::convert::Infallible;

pub const HTTP_REQUEST_CLASS_NAME: &str = "HttpServer\\HttpRequest";

pub static HTTP_REQUEST_CLASS: StaticStateClass<()> = StaticStateClass::null();

/// Register the class `HttpServer\HttpRequest` by `ClassEntity`.
pub fn make_request_class() -> ClassEntity<()> {
    let mut class = ClassEntity::new(HTTP_REQUEST_CLASS_NAME);

    // The state class will be initialized after class registered.
    class.bind(&HTTP_REQUEST_CLASS);

    // Register the http headers field with public visibility.
    class.add_property("headers", Visibility::Public, ());

    // Register the http body field with public visibility.
    class.add_property("data", Visibility::Public, ());

    // Register the constructor method with public visibility, initialize the
    // headers with empty array.
    class.add_method("__construct", Visibility::Public, |this, _arguments| {
        this.set_property("headers", ZArray::new());
        Ok::<_, Infallible>(())
    });

    class
}

/// Instantiate the object with class `HttpServer\HttpRequest`.
pub fn new_request_object() -> phper::Result<StateObject<()>> {
    HTTP_REQUEST_CLASS.new_object([])
}

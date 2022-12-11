// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::classes::{ClassEntity, Visibility};

pub const HTTP_REQUEST_CLASS_NAME: &str = "HttpServer\\HttpRequest";

pub fn make_request_class() -> ClassEntity<()> {
    let mut class = ClassEntity::new(HTTP_REQUEST_CLASS_NAME);

    class.add_property("header", Visibility::Public, ());
    class.add_property("server", Visibility::Public, ());
    class.add_property("data", Visibility::Private, ());

    class
}

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
    classes::{ClassEntity, ClassEntry},
    errors::{exception_class, Throwable},
};
use std::error::Error;

const EXCEPTION_CLASS_NAME: &str = "HttpServer\\HttpServerException";

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct HttpServerError(pub Box<dyn Error>);

impl Throwable for HttpServerError {
    fn get_class(&self) -> &ClassEntry {
        ClassEntry::from_globals(EXCEPTION_CLASS_NAME).unwrap_or_else(|_| exception_class())
    }
}

impl From<HttpServerError> for phper::Error {
    fn from(e: HttpServerError) -> Self {
        phper::Error::throw(e)
    }
}

pub fn make_exception_class() -> ClassEntity<()> {
    let mut class = ClassEntity::new(EXCEPTION_CLASS_NAME);
    class.extends(exception_class);
    class
}

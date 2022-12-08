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
    classes::{ClassEntry, StatefulClass},
    errors::{exception_class, Throwable},
};

/// The exception class name of extension.
const EXCEPTION_CLASS_NAME: &str = "HttpClient\\HttpClientException";

pub fn make_exception_class() -> StatefulClass<()> {
    let mut exception_class = StatefulClass::new(EXCEPTION_CLASS_NAME);
    // The `extends` is same as the PHP class `extends`.
    exception_class.extends("Exception");
    exception_class
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ReqwestError(pub reqwest::Error);

impl Throwable for ReqwestError {
    fn get_class(&self) -> &ClassEntry {
        ClassEntry::from_globals(EXCEPTION_CLASS_NAME).unwrap_or_else(|_| exception_class())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("should call '{method_name}()' before call 'body()'")]
pub struct ResponseAfterRead {
    pub method_name: String,
}

impl Throwable for ResponseAfterRead {
    fn get_class(&self) -> &ClassEntry {
        ClassEntry::from_globals(EXCEPTION_CLASS_NAME).unwrap_or_else(|_| exception_class())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("should not call 'body()' multi time")]
pub struct ResponseHadRead;

impl Throwable for ResponseHadRead {
    fn get_class(&self) -> &ClassEntry {
        ClassEntry::from_globals(EXCEPTION_CLASS_NAME).unwrap_or_else(|_| exception_class())
    }
}

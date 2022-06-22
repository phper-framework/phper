// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::classes::{ClassEntry, StatefulClass};

const EXCEPTION_CLASS_NAME: &str = "HttpClient\\HttpClientException";

#[derive(Debug, thiserror::Error, phper::Throwable)]
#[throwable_class(EXCEPTION_CLASS_NAME)]
pub enum HttpClientError {
    #[error(transparent)]
    #[throwable(transparent)]
    Phper(#[from] phper::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("should call '{method_name}()' before call 'body()'")]
    ResponseAfterRead { method_name: String },

    #[error("should not call 'body()' multi time")]
    ResponseHadRead,
}

pub fn make_exception_class() -> StatefulClass<()> {
    let mut exception_class = StatefulClass::new(EXCEPTION_CLASS_NAME);
    exception_class.extends("Exception");
    exception_class
}

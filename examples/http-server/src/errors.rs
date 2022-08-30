// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use hyper::header::{InvalidHeaderName, InvalidHeaderValue};
use phper::classes::{ClassEntry, StatefulClass};
use std::{net::AddrParseError, str::Utf8Error};

const EXCEPTION_CLASS_NAME: &str = "HttpServer\\HttpServerException";

#[derive(Debug, thiserror::Error, phper::Throwable)]
#[throwable_class(EXCEPTION_CLASS_NAME)]
pub enum HttpServerError {
    #[error(transparent)]
    #[throwable(transparent)]
    Phper(#[from] phper::Error),

    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),

    #[error(transparent)]
    AddrParse(#[from] AddrParseError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Hyper(#[from] hyper::Error),

    #[error(transparent)]
    InvalidHeaderName(#[from] InvalidHeaderName),

    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
}

pub fn make_exception_class() -> StatefulClass<()> {
    let mut exception_class = StatefulClass::new(EXCEPTION_CLASS_NAME);
    exception_class.extends("Exception");
    exception_class
}

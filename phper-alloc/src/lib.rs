// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![warn(rust_2018_idioms, missing_docs)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/112468984?s=200&v=4")]

use std::borrow::Borrow;

/// Duplicate an object without deep copy, but to only add the refcount, for php
/// refcount struct.
pub trait ToRefOwned {
    /// The resulting type after obtaining ownership.
    type Owned: Borrow<Self>;

    /// Creates owned data from borrowed data, by increasing refcount.
    fn to_ref_owned(&mut self) -> Self::Owned;
}

/// Duplicate an object without deep copy, but to only add the refcount, for php
/// refcount struct.
pub trait RefClone {
    /// Returns a refcount value with same reference of the value.
    fn ref_clone(&mut self) -> Self;
}

// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![warn(rust_2018_idioms, clippy::dbg_macro)]
#![doc = include_str!("../README.md")]

#[doc = include_str!("../doc/_01_introduction/index.md")]
pub mod _01_introduction {}

#[doc = include_str!("../doc/_02_quick_start/index.md")]
pub mod _02_quick_start {

    #[doc = include_str!("../doc/_02_quick_start/_01_write_your_first_extension/index.md")]
    pub mod _01_write_your_first_extension {}

    #[doc = include_str!("../doc/_02_quick_start/_02_write_a_simple_http_client/index.md")]
    pub mod _02_write_a_simple_http_client {}
}

/// TODO
pub mod _03_integrate_with_pecl {}

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
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use phper;

#[doc = include_str!("../doc/_01_introduction/index.md")]
pub mod _01_introduction {}

#[doc = include_str!("../doc/_02_quick_start/index.md")]
pub mod _02_quick_start {

    #[doc = include_str!("../doc/_02_quick_start/_01_write_your_first_extension/index.md")]
    pub mod _01_write_your_first_extension {}

    #[doc = include_str!("../doc/_02_quick_start/_02_write_a_simple_http_client/index.md")]
    pub mod _02_write_a_simple_http_client {}
}

#[doc = include_str!("../doc/_03_integrate_with_pecl/index.md")]
pub mod _03_integrate_with_pecl {}

#[doc = include_str!("../doc/_04_zval/index.md")]
pub mod _04_zval {}

#[doc = include_str!("../doc/_05_internal_types/index.md")]
pub mod _05_internal_types {

    #[doc = include_str!("../doc/_05_internal_types/_01_z_str/index.md")]
    pub mod _01_z_str {}

    #[doc = include_str!("../doc/_05_internal_types/_02_z_arr/index.md")]
    pub mod _02_z_arr {}

    #[doc = include_str!("../doc/_05_internal_types/_03_z_obj/index.md")]
    pub mod _03_z_obj {}
}

#[doc = include_str!("../doc/_06_module/index.md")]
pub mod _06_module {

    #[doc = include_str!("../doc/_06_module/_01_hooks/index.md")]
    pub mod _01_hooks {}

    #[doc = include_str!("../doc/_06_module/_02_register_functions/index.md")]
    pub mod _02_register_functions {}

    #[doc = include_str!("../doc/_06_module/_03_register_constants/index.md")]
    pub mod _03_register_constants {}

    #[doc = include_str!("../doc/_06_module/_04_register_ini_settings/index.md")]
    pub mod _04_register_ini_settings {}

    #[doc = include_str!("../doc/_06_module/_05_extension_information/index.md")]
    pub mod _05_extension_information {}

    #[doc = include_str!("../doc/_06_module/_06_register_class/index.md")]
    pub mod _06_register_class {}

    #[doc = include_str!("../doc/_06_module/_07_register_interface/index.md")]
    pub mod _07_register_interface {}

    #[cfg(all(phper_major_version = "8", not(phper_minor_version = "0")))]
    #[doc = include_str!("../doc/_06_module/_08_register_enum/index.md")]
    pub mod _08_register_enum {}
}

/// TODO
pub mod _07_allocation {}

/// TODO
pub mod _08_handle_exception {}

/// TODO
pub mod _09_build_script {}

/// TODO
pub mod _10_integration_tests {}

/// TODO
pub mod _11_macros {}

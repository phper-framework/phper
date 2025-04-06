// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

#![cfg(phper_enum_supported)]

use phper::{classes::Visibility, enums::EnumEntity, modules::Module};
use std::convert::Infallible;

pub fn integrate(module: &mut Module) {
    // Create pure enum (without backing type)
    create_pure_enum(module);

    // Create int-backed enum
    create_int_backed_enum(module);

    // Create string-backed enum
    create_string_backed_enum(module);
}

fn create_pure_enum(module: &mut Module) {
    let mut enum_entity = EnumEntity::new("IntegrationTest\\PureEnum");

    // Add enum cases
    enum_entity.add_case("ONE", ());
    enum_entity.add_case("TWO", ());
    enum_entity.add_case("THREE", ());

    // Add constants
    enum_entity.add_constant("VERSION", "1.0.0");

    // Add static method
    enum_entity.add_static_method("getDescription", Visibility::Public, |_| {
        Ok::<_, Infallible>("Pure enum implementation")
    });

    module.add_enum(enum_entity);
}

fn create_int_backed_enum(module: &mut Module) {
    let mut enum_entity = EnumEntity::<i64>::new("IntegrationTest\\IntEnum");

    // Add enum cases with integer values
    enum_entity.add_case("LOW", 1);
    enum_entity.add_case("MEDIUM", 5);
    enum_entity.add_case("HIGH", 10);

    module.add_enum(enum_entity);
}

fn create_string_backed_enum(module: &mut Module) {
    let mut enum_entity = EnumEntity::<String>::new("IntegrationTest\\StringEnum");

    // Add enum cases with string values
    enum_entity.add_case("RED", "FF0000".to_string());
    enum_entity.add_case("GREEN", "00FF00".to_string());
    enum_entity.add_case("BLUE", "0000FF".to_string());

    module.add_enum(enum_entity);
}

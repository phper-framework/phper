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

use phper::{
    alloc::ToRefOwned,
    classes::Visibility,
    enums::{Enum, EnumEntity},
    modules::Module,
    objects::ZObject,
};
use std::convert::Infallible;

pub fn integrate(module: &mut Module) {
    // Create pure enum (without backing type)
    create_pure_enum(module);

    // Create int-backed enum
    create_int_backed_enum(module);

    // Create string-backed enum
    create_string_backed_enum(module);

    // Add test function for Enum::from_name
    test_enum_from_name(module);
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

// Add test function to test Enum::from_name and get_case functionality, and
// convert the result to ZObject
fn test_enum_from_name(module: &mut Module) {
    // Test getting a case from pure enum
    module.add_function("test_enum_from_name_pure", |_args| {
        // Use Enum::from_name to get the created enum
        let mut pure_enum = Enum::from_name("IntegrationTest\\PureEnum");

        // Test the get_case method and convert the result to ZObject
        let one_case = pure_enum.get_mut_case("ONE")?;
        let zobj: ZObject = one_case.to_ref_owned();

        Ok::<_, phper::Error>(zobj)
    });

    // Test getting a case from int-backed enum
    module.add_function("test_enum_from_name_int", |_args| {
        // Use Enum::from_name to get the created enum
        let mut int_enum = Enum::from_name("IntegrationTest\\IntEnum");

        // Test the get_case method and convert the result to ZObject
        let low_case = int_enum.get_mut_case("LOW")?;
        let zobj: ZObject = low_case.to_ref_owned();

        Ok::<_, phper::Error>(zobj)
    });

    // Test getting a case from string-backed enum
    module.add_function("test_enum_from_name_string", |_args| {
        // Use Enum::from_name to get the created enum
        let mut string_enum = Enum::from_name("IntegrationTest\\StringEnum");

        // Test the get_case method and convert the result to ZObject
        let red_case = string_enum.get_mut_case("RED")?;
        let zobj: ZObject = red_case.to_ref_owned();

        Ok::<_, phper::Error>(zobj)
    });
}

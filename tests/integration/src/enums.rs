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

    // Add test for EnumCase
    test_enum_case(module);
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
        let one_case = unsafe { pure_enum.get_mut_case("ONE")? };
        let zobj: ZObject = one_case.to_ref_owned();

        Ok::<_, phper::Error>(zobj)
    });

    // Test getting a case from int-backed enum
    module.add_function("test_enum_from_name_int", |_args| {
        // Use Enum::from_name to get the created enum
        let mut int_enum = Enum::from_name("IntegrationTest\\IntEnum");

        // Test the get_case method and convert the result to ZObject
        let low_case = unsafe { int_enum.get_mut_case("LOW")? };
        let zobj: ZObject = low_case.to_ref_owned();

        Ok::<_, phper::Error>(zobj)
    });

    // Test getting a case from string-backed enum
    module.add_function("test_enum_from_name_string", |_args| {
        // Use Enum::from_name to get the created enum
        let mut string_enum = Enum::from_name("IntegrationTest\\StringEnum");

        // Test the get_case method and convert the result to ZObject
        let red_case = unsafe { string_enum.get_mut_case("RED")? };
        let zobj: ZObject = red_case.to_ref_owned();

        Ok::<_, phper::Error>(zobj)
    });
}

// Add test function for EnumCase
fn test_enum_case(module: &mut Module) {
    // First create and register all test enums
    // Create pure enum
    let mut pure_enum_entity = EnumEntity::new("IntegrationTest\\TestPureEnum");
    let one_case = pure_enum_entity.add_case("ONE", ());
    let _two_case = pure_enum_entity.add_case("TWO", ());
    let _pure_enum = module.add_enum(pure_enum_entity);

    // Create integer-backed enum
    let mut int_enum_entity = EnumEntity::<i64>::new("IntegrationTest\\TestIntEnum");
    let low_case = int_enum_entity.add_case("LOW", 10);
    let _high_case = int_enum_entity.add_case("HIGH", 100);
    let _int_enum = module.add_enum(int_enum_entity);

    // Create string-backed enum
    let mut string_enum_entity = EnumEntity::<String>::new("IntegrationTest\\TestStringEnum");
    let red_case = string_enum_entity.add_case("RED", "red".to_string());
    let _blue_case = string_enum_entity.add_case("BLUE", "blue".to_string());
    let _string_enum = module.add_enum(string_enum_entity);

    // Now use previously created EnumCase instances in closures

    // Create EnumCase from Pure Enum
    module.add_function("test_enum_case_pure", {
        move |_args| {
            // Test that we can use the EnumCase to get the case directly
            let mut one_case = one_case.clone();
            let one_obj = one_case.get_mut_case();
            let one_zobj: ZObject = one_obj.to_ref_owned();

            // Return the object for PHP side verification
            phper::ok(one_zobj)
        }
    });

    // Create EnumCase from Int Backed Enum
    module.add_function("test_enum_case_int", {
        move |_args| {
            // Test that we can use the EnumCase to get the case directly
            let mut low_case = low_case.clone();
            let low_obj = low_case.get_mut_case();
            let low_zobj: ZObject = low_obj.to_ref_owned();

            // Return the object for PHP side verification
            phper::ok(low_zobj)
        }
    });

    // Create EnumCase from String Backed Enum
    module.add_function("test_enum_case_string", {
        move |_args| {
            // Test EnumCase methods
            let mut red_case = red_case.clone();
            let red_obj = red_case.get_mut_case();
            let red_zobj: ZObject = red_obj.to_ref_owned();

            // Return the object for PHP side verification
            phper::ok(red_zobj)
        }
    });
}

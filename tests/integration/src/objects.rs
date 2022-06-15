// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{
    classes::StatelessClassEntry, modules::Module, objects::Object, types::TypeInfo, values::ZVal,
};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_objects_new_drop",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let o = Object::new_by_std_class();
            drop(o);
            Ok(())
        },
        vec![],
    );

    module.add_function(
        "integrate_objects_get_set",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let mut o = Object::new_by_std_class();

            o.set_property("foo", ZVal::from("bar"));
            let val = o.get_property("foo");
            assert_eq!(val.expect_z_str()?.to_str()?, "bar");

            let not_exists = o.get_property("no_exists");
            not_exists.expect_null()?;

            Ok(())
        },
        vec![],
    );

    module.add_function(
        "integrate_objects_set_val",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let o = Object::new_by_std_class();
            let v = &mut ZVal::default();
            *v = o.into();
            assert_eq!(v.get_type_info(), TypeInfo::OBJECT);
            Ok(())
        },
        vec![],
    );

    module.add_function(
        "integrate_objects_call",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let mut o = StatelessClassEntry::from_globals("Exception")?
                .new_object(&mut [ZVal::from("What's happen?")])?;
            let message = o.call("getMessage", &mut [])?;
            assert_eq!(message.expect_z_str()?.to_str()?, "What's happen?");
            Ok(())
        },
        vec![],
    );
}

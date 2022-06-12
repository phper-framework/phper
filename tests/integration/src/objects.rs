// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{classes::StatelessClassEntry, modules::Module, objects::Object, values::ZVal};

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
            assert_eq!(val.to_string()?, "bar");

            let not_exists = o.get_property("no_exists");
            not_exists.as_null()?;

            Ok(())
        },
        vec![],
    );

    module.add_function(
        "integrate_objects_set_val",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let o = Object::new_by_std_class();
            let mut v = ZVal::null();
            v.set(o);
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
            assert_eq!(message.to_string()?, "What's happen?");
            Ok(())
        },
        vec![],
    );
}

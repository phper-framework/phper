// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{
    alloc::{RefClone, ToRefOwned},
    classes::{ClassEntity, ClassEntry, Visibility},
    functions::Argument,
    modules::Module,
    objects::ZObject,
    types::TypeInfo,
    values::ZVal,
};
use std::convert::Infallible;

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_objects_new_drop",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let o = ZObject::new_by_std_class();
            drop(o);
            Ok(())
        },
    );

    module.add_function(
        "integrate_objects_get_set",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let mut o = ZObject::new_by_std_class();

            o.set_property("foo", ZVal::from("bar"));
            let val = o.get_property("foo");
            assert_eq!(val.expect_z_str()?.to_str()?, "bar");

            let not_exists = o.get_property("no_exists");
            not_exists.expect_null()?;

            Ok(())
        },
    );

    module.add_function(
        "integrate_objects_set_val",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let o = ZObject::new_by_std_class();
            let v = &mut ZVal::default();
            *v = o.into();
            assert_eq!(v.get_type_info().get_base_type(), TypeInfo::OBJECT);
            Ok(())
        },
    );

    module.add_function(
        "integrate_objects_call",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let mut o = ClassEntry::from_globals("Exception")?
                .new_object(&mut [ZVal::from("What's happen?")])?;
            let message = o.call("getMessage", &mut [])?;
            assert_eq!(message.expect_z_str()?.to_str()?, "What's happen?");
            Ok(())
        },
    );

    module
        .add_function(
            "integrate_objects_to_ref_owned",
            |arguments: &mut [ZVal]| -> phper::Result<()> {
                let o1 = arguments[0].expect_mut_z_obj()?;

                o1.set_property("foo", "bar");

                let mut o2 = o1.to_ref_owned();
                assert_eq!(
                    o2.get_property("foo").as_z_str().unwrap().to_bytes(),
                    b"bar"
                );

                o2.set_property("foo", "baz");
                assert_eq!(
                    o1.get_property("foo").as_z_str().unwrap().to_bytes(),
                    b"baz"
                );
                assert_eq!(
                    o2.get_property("foo").as_z_str().unwrap().to_bytes(),
                    b"baz"
                );

                Ok(())
            },
        )
        .argument(Argument::by_val("obj"));

    module
        .add_function(
            "integrate_objects_to_ref_clone",
            |arguments: &mut [ZVal]| -> phper::Result<()> {
                let o1 = arguments[0].expect_mut_z_obj()?;

                o1.set_property("foo", "bar");

                let mut o2 = o1.to_ref_owned();
                let o3 = o2.ref_clone();

                o1.set_property("foo", "baz");

                assert_eq!(
                    o1.get_property("foo").as_z_str().unwrap().to_bytes(),
                    b"baz"
                );
                assert_eq!(
                    o2.get_property("foo").as_z_str().unwrap().to_bytes(),
                    b"baz"
                );
                assert_eq!(
                    o3.get_property("foo").as_z_str().unwrap().to_bytes(),
                    b"baz"
                );

                Ok(())
            },
        )
        .argument(Argument::by_val("obj"));

    module.add_function("integrate_objects_set_props", |_| {
        let mut o = ZObject::new_by_std_class();

        o.set_property("foo", "bar");
        assert_eq!(o.get_property("foo").as_z_str().unwrap().to_bytes(), b"bar");

        o.set_property("foo", ());
        assert_eq!(o.get_property("foo").as_null(), Some(()));

        o.set_property("foo", true);
        assert_eq!(o.get_property("foo").as_bool(), Some(true));

        o.set_property("foo", ZVal::from(100i64));
        assert_eq!(o.get_property("foo").as_long(), Some(100i64));

        phper::ok(())
    });

    let class_a =
        ClassEntity::new_with_state_constructor("IntegrationTest\\Objects\\A", || 123456i64);
    module.add_class(class_a);

    let mut class_b =
        ClassEntity::new_with_state_constructor("IntegrationTest\\Objects\\B", || 123456i64);
    class_b.state_cloner(Clone::clone);
    class_b.add_method("incr", Visibility::Public, |this, _| {
        *this.as_mut_state() += 1;
        Ok::<_, Infallible>(())
    });
    class_b.add_method("get", Visibility::Public, |this, _| {
        Ok::<_, Infallible>(*this.as_state())
    });
    module.add_class(class_b);
}

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
    arrays::{InsertKey, IterKey, ZArray},
    modules::Module,
    objects::{ZObj, ZObject},
    strings::ZString,
    values::ZVal,
};

pub fn integrate(module: &mut Module) {
    module.add_function(
        "integrate_arrays_new_drop",
        |_: &mut [ZVal]| -> phper::Result<String> {
            let mut a1 = ZArray::new();
            a1.insert("foo", ZVal::from("FOO"));
            let val = a1.get("foo").unwrap();
            let val = val.as_z_str().unwrap().to_str().unwrap();

            let mut a2 = ZArray::new();
            a2.insert("bar", ZVal::from("BAR"));
            let bar = a2.get("bar").unwrap();
            let bar = bar.as_z_str().unwrap().to_str().unwrap();

            Ok(format!("{} {}", val, bar))
        },
    );

    module.add_function(
        "integrate_arrays_types",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let mut a = ZArray::new();

            a.insert(0, ZVal::from(0));
            a.insert(1, ZVal::from(1));
            a.insert("foo", ZVal::from("bar"));
            a.insert(
                "arr",
                ZVal::from({
                    let mut arr = ZArray::new();
                    arr.insert(0, ZVal::from(0));
                    arr.insert(1, ZVal::from(1));
                    arr
                }),
            );
            a.insert(
                "obj",
                ZVal::from({
                    let mut o = ZObject::new_by_std_class();
                    o.set_property("foo", ZVal::from("bar"));
                    o
                }),
            );

            assert_eq!(a.get(0).unwrap().as_long().unwrap(), 0);
            assert_eq!(a.get(1).unwrap().as_long().unwrap(), 1);
            assert_eq!(
                a.get("foo").unwrap().as_z_str().unwrap().to_str().unwrap(),
                "bar"
            );

            let arr = a.get("arr").unwrap().as_z_arr().unwrap();
            assert_eq!(arr.get(0).unwrap().as_long().unwrap(), 0);
            assert_eq!(arr.get(1).unwrap().as_long().unwrap(), 1);

            let obj: &mut ZObj = a.get_mut("obj").unwrap().expect_mut_z_obj()?;
            let val = obj.get_property("foo");
            assert_eq!(val.as_z_str().unwrap().to_str().unwrap(), "bar");

            assert!(a.get(10).is_none());
            assert!(a.get("not_exists").is_none());

            Ok(())
        },
    );

    module.add_function(
        "integrate_arrays_insert",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let mut a = ZArray::new();
            assert_eq!(a.len(), 0);

            a.insert(InsertKey::NextIndex, ZVal::from("0"));
            assert_eq!(a.get(0).unwrap().as_z_str().unwrap().to_str(), Ok("0"));
            assert_eq!(a.len(), 1);

            a.insert(10, ZVal::from("10"));
            assert_eq!(a.get(10).unwrap().as_z_str().unwrap().to_str(), Ok("10"));
            assert_eq!(a.len(), 2);

            a.insert(10, ZVal::from("foo"));
            assert_eq!(a.get(10).unwrap().as_z_str().unwrap().to_str(), Ok("foo"));
            assert_eq!(a.len(), 2);

            a.insert((), ZVal::from("11"));
            assert_eq!(a.get(11).unwrap().as_z_str().unwrap().to_str(), Ok("11"));
            assert_eq!(a.len(), 3);

            a.insert((), ZVal::from("12"));
            assert_eq!(a.get(12).unwrap().as_z_str().unwrap().to_str(), Ok("12"));
            assert_eq!(a.len(), 4);

            a.insert("foo", ZVal::from("bar"));
            assert_eq!(
                a.get("foo").unwrap().as_z_str().unwrap().to_str(),
                Ok("bar")
            );
            assert_eq!(a.len(), 5);

            a.insert("foo", ZVal::from("bar2"));
            assert_eq!(
                a.get("foo").unwrap().as_z_str().unwrap().to_str(),
                Ok("bar2")
            );
            assert_eq!(a.len(), 5);

            assert!(a.get(13).is_none());
            assert_eq!(a.len(), 5);

            Ok(())
        },
    );

    module.add_function(
        "integrate_arrays_exists",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let mut a = ZArray::new();

            assert!(!a.exists("foo"));

            a.insert("foo", ZVal::from("bar"));
            assert!(a.exists("foo"));

            Ok(())
        },
    );

    module.add_function(
        "integrate_arrays_remove",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let mut a = ZArray::new();

            a.insert(10, ZVal::from(10));
            a.insert("foo", ZVal::from("bar"));

            assert!(a.exists(10));
            assert!(a.remove(10));
            assert!(!a.exists(10));
            assert!(!a.remove(10));

            assert!(a.exists("foo"));
            assert!(a.remove("foo"));
            assert!(!a.exists("foo"));
            assert!(!a.remove("foo"));

            Ok(())
        },
    );

    module.add_function(
        "integrate_arrays_clone",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let mut a = ZArray::new();

            a.insert(0, ZVal::from(0));
            a.insert((), ZVal::from(1));
            a.insert("foo", ZVal::from("bar"));

            let b = a.clone();
            assert_eq!(b.get(0).unwrap().as_long(), Some(0));
            assert_eq!(b.get(1).unwrap().as_long(), Some(1));
            assert_eq!(
                b.get("foo").unwrap().as_z_str().unwrap().to_str(),
                Ok("bar")
            );

            Ok(())
        },
    );

    module.add_function(
        "integrate_arrays_for_each",
        |_: &mut [ZVal]| -> phper::Result<()> {
            let mut a = ZArray::new();

            a.insert(0, ZVal::from(0));
            a.insert((), ZVal::from(1));
            a.insert("foo", ZVal::from("bar"));

            let mut it = a.iter();
            {
                let (k, v) = it.next().unwrap();
                assert_eq!(k, 0.into());
                assert_eq!(v.as_long(), Some(0));
            }
            {
                let (k, v) = it.next().unwrap();
                assert_eq!(k, 1.into());
                assert_eq!(v.as_long(), Some(1));
            }
            {
                let (k, v) = it.next().unwrap();
                assert_eq!(k, IterKey::ZStr(&ZString::new("foo")));
                assert_eq!(v.as_z_str().unwrap().to_str(), Ok("bar"));
            }
            {
                assert!(it.next().is_none());
            }
            {
                assert!(it.next().is_none());
            }

            let mut it = a.iter_mut();
            {
                let (k, v) = it.next().unwrap();
                assert_eq!(k, 0.into());
                assert_eq!(v.as_long(), Some(0));
                *v.as_mut_long().unwrap() += 100;
            }
            {
                let (k, v) = it.next().unwrap();
                assert_eq!(k, 1.into());
                assert_eq!(v.as_long(), Some(1));
                *v.as_mut_long().unwrap() += 100;
            }
            {
                let (k, v) = it.next().unwrap();
                assert_eq!(k, IterKey::ZStr(&ZString::new("foo")));
                assert_eq!(v.as_z_str().unwrap().to_str(), Ok("bar"));
            }
            {
                assert!(it.next().is_none());
            }
            {
                assert!(it.next().is_none());
            }

            assert_eq!(a.get(0).unwrap().as_long(), Some(100));
            assert_eq!(a.get(1).unwrap().as_long(), Some(101));

            Ok(())
        },
    );
}

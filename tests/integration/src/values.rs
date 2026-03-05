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
    arrays::{InsertKey, ZArray},
    modules::Module,
    objects::ZObject,
    values::{ZVal, ZValMut, ZValRef},
};
use std::convert::Infallible;

pub fn integrate(module: &mut Module) {
    integrate_returns(module);
}

fn integrate_returns(module: &mut Module) {
    module.add_function(
        "integration_values_return_null",
        integration_values_return_null,
    );
    module.add_function(
        "integration_values_return_true",
        integration_values_return_true,
    );
    module.add_function(
        "integration_values_return_false",
        integration_values_return_false,
    );
    module.add_function(
        "integration_values_return_i64",
        integration_values_return_i64,
    );
    module.add_function(
        "integration_values_return_f64",
        integration_values_return_f64,
    );
    module.add_function(
        "integration_values_return_str",
        integration_values_return_str,
    );
    module.add_function(
        "integration_values_return_string",
        integration_values_return_string,
    );
    module.add_function(
        "integration_values_return_array",
        integration_values_return_array,
    );
    module.add_function(
        "integration_values_return_kv_array",
        integration_values_return_kv_array,
    );
    module.add_function(
        "integration_values_return_object",
        integration_values_return_object,
    );
    module.add_function(
        "integration_values_return_option_i64_some",
        integration_values_return_option_i64_some,
    );
    module.add_function(
        "integration_values_return_option_i64_none",
        integration_values_return_option_i64_none,
    );
    module.add_function(
        "integration_values_return_result_string_ok",
        integration_values_return_result_string_ok,
    );
    module.add_function(
        "integration_values_return_result_string_err",
        integration_values_return_result_string_err,
    );
    module.add_function(
        "integration_values_return_val",
        integration_values_return_val,
    );
    module.add_function("integration_values_as", integration_values_as);
}

fn integration_values_return_null(_: &mut [ZVal]) -> Result<(), Infallible> {
    Ok(())
}

fn integration_values_return_true(_: &mut [ZVal]) -> Result<bool, Infallible> {
    Ok(true)
}

fn integration_values_return_false(_: &mut [ZVal]) -> Result<bool, Infallible> {
    Ok(false)
}

fn integration_values_return_i64(_: &mut [ZVal]) -> Result<i64, Infallible> {
    Ok(64)
}

fn integration_values_return_f64(_: &mut [ZVal]) -> Result<f64, Infallible> {
    Ok(64.0)
}

fn integration_values_return_str(_: &mut [ZVal]) -> Result<&'static str, Infallible> {
    Ok("foo")
}

fn integration_values_return_string(_: &mut [ZVal]) -> Result<String, Infallible> {
    Ok("foo".to_string())
}

fn integration_values_return_array(_: &mut [ZVal]) -> Result<ZArray, Infallible> {
    let mut arr = ZArray::new();
    arr.insert(InsertKey::NextIndex, ZVal::from("a"));
    arr.insert(InsertKey::NextIndex, ZVal::from("b"));
    arr.insert(InsertKey::NextIndex, ZVal::from("c"));
    Ok(arr)
}

fn integration_values_return_kv_array(_: &mut [ZVal]) -> Result<ZArray, Infallible> {
    let mut arr = ZArray::new();
    arr.insert("a", ZVal::from(1));
    arr.insert("b", ZVal::from("foo"));
    Ok(arr)
}

fn integration_values_return_object(_: &mut [ZVal]) -> Result<ZObject, Infallible> {
    let mut object = ZObject::new_by_std_class();
    object.set_property("foo", ZVal::from("bar"));
    Ok(object)
}

fn integration_values_return_option_i64_some(_: &mut [ZVal]) -> Result<Option<i64>, Infallible> {
    Ok(Some(64))
}

fn integration_values_return_option_i64_none(_: &mut [ZVal]) -> Result<Option<i64>, Infallible> {
    Ok(None)
}

fn integration_values_return_result_string_ok(
    _: &mut [ZVal],
) -> phper::Result<impl Into<ZVal> + use<>> {
    Ok("foo".to_string())
}

fn integration_values_return_result_string_err(_: &mut [ZVal]) -> phper::Result<()> {
    Err(phper::Error::Boxed("a zhe".into()))
}

fn integration_values_return_val(_: &mut [ZVal]) -> Result<ZVal, Infallible> {
    Ok(ZVal::from("foo"))
}

fn integration_values_as(_: &mut [ZVal]) -> Result<(), Infallible> {
    {
        let val = ZVal::default();
        assert_eq!(val.as_null(), Some(()));
        assert_eq!(val.as_long(), None);
    }

    {
        let val = ZVal::from(true);
        assert_eq!(val.as_bool(), Some(true));
        assert_eq!(val.as_long(), None);
    }

    {
        let mut val = ZVal::from(100i64);
        assert_eq!(val.as_long(), Some(100));
        assert_eq!(val.as_double(), None);
        if let Some(l) = val.as_mut_long() {
            *l += 100;
        }
        assert_eq!(val.as_long(), Some(200));
    }

    {
        let mut val = ZVal::from(100f64);
        assert_eq!(val.as_double(), Some(100.));
        assert_eq!(val.as_long(), None);
        if let Some(d) = val.as_mut_double() {
            *d += 100.;
        }
        assert_eq!(val.as_double(), Some(200.));
    }

    {
        let val = ZVal::default();
        assert_eq!(val.expect_type::<()>().ok(), Some(()));
        assert!(val.expect_type::<()>().is_ok());
        assert!(val.expect_type::<bool>().is_err());
    }

    {
        let val = ZVal::from(true);
        assert_eq!(val.expect_type::<bool>().ok(), Some(true));
        assert_eq!(val.expect_type::<i64>().ok(), None);
        assert!(val.expect_type::<i64>().is_err());
    }

    {
        let mut val = ZVal::from(100i64);
        assert_eq!(val.expect_type::<i64>().ok(), Some(100));
        if let Some(l) = val.expect_mut_type::<&mut i64>().ok() {
            *l += 100;
        }
        assert_eq!(val.expect_type::<i64>().unwrap(), 200);
        assert!(val.expect_mut_type::<&mut f64>().is_err());
    }

    {
        let mut val = ZVal::from(100f64);
        assert_eq!(val.expect_type::<f64>().ok(), Some(100.));
        if let Some(d) = val.expect_mut_type::<&mut f64>().ok() {
            *d += 100.;
        }
        assert_eq!(val.expect_type::<f64>().unwrap(), 200.);
        assert!(val.expect_mut_type::<&mut i64>().is_err());
    }

    {
        let val = ZVal::from("foo");
        assert_eq!(
            val.expect_type::<&phper::strings::ZStr>().unwrap().to_bytes(),
            b"foo"
        );
        assert!(val.expect_type::<&phper::arrays::ZArr>().is_err());
    }

    {
        let mut arr = ZArray::new();
        arr.insert(InsertKey::NextIndex, ZVal::from("a"));
        arr.insert(InsertKey::NextIndex, ZVal::from("b"));
        let mut val = ZVal::from(arr);

        {
            let zarr = val.expect_type::<&phper::arrays::ZArr>().unwrap();
            let got = zarr.get(1).unwrap().expect_z_str().unwrap().to_bytes();
            assert_eq!(got, b"b");
        }

        {
            let zarr = val.expect_mut_type::<&mut phper::arrays::ZArr>().unwrap();
            zarr.insert(InsertKey::NextIndex, ZVal::from("c"));
            assert_eq!(zarr.len(), 3);
        }

        assert!(val.expect_type::<&phper::objects::ZObj>().is_err());
    }

    {
        let mut obj = ZObject::new_by_std_class();
        obj.set_property("foo", ZVal::from("bar"));
        let mut val = ZVal::from(obj);

        {
            let zobj = val.expect_type::<&phper::objects::ZObj>().unwrap();
            let got = zobj.get_property("foo").expect_z_str().unwrap().to_bytes();
            assert_eq!(got, b"bar");
        }

        {
            let zobj = val.expect_mut_type::<&mut phper::objects::ZObj>().unwrap();
            zobj.set_property("foo", ZVal::from("baz"));
        }

        let got = val
            .expect_type::<&phper::objects::ZObj>()
            .unwrap()
            .get_property("foo")
            .expect_z_str()
            .unwrap()
            .to_bytes();
        assert_eq!(got, b"baz");

        assert!(val.expect_type::<&phper::arrays::ZArr>().is_err());
    }

    {
        let val = ZVal::default();
        match val.to_value().unwrap() {
            ZValRef::Null => {}
            other => panic!("expect Null, got {other:?}"),
        }
    }

    {
        let val = ZVal::from("foo");
        match ZValRef::from_z_val(&val).unwrap() {
            ZValRef::Str(s) => assert_eq!(s.to_bytes(), b"foo"),
            other => panic!("expect Str, got {other:?}"),
        }
    }

    {
        let mut val = ZVal::default();
        match val.to_value_mut().unwrap() {
            ZValMut::Null => {}
            other => panic!("expect Null, got {other:?}"),
        }
    }

    {
        let mut val = ZVal::from(100i64);
        match ZValMut::from_z_val_mut(&mut val).unwrap() {
            ZValMut::Long(i) => *i += 23,
            other => panic!("expect Long, got {other:?}"),
        }
        assert_eq!(val.expect_long().unwrap(), 123);
    }

    {
        let mut val = ZVal::from("bar");
        match val.to_value_mut().unwrap() {
            ZValMut::Str(s) => {
                assert_eq!(s.to_bytes(), b"bar");
            }
            other => panic!("expect Str, got {other:?}"),
        }
    }

    {
        let mut arr = ZArray::new();
        arr.insert(InsertKey::NextIndex, ZVal::from("x"));
        let val = ZVal::from(arr);

        match val.to_value().unwrap() {
            ZValRef::Arr(a) => {
                let got = a.get(0).unwrap().expect_z_str().unwrap().to_bytes();
                assert_eq!(got, b"x");
            }
            other => panic!("expect Arr, got {other:?}"),
        }
    }

    {
        let mut obj = ZObject::new_by_std_class();
        obj.set_property("name", ZVal::from("copilot"));
        let val = ZVal::from(obj);

        match val.to_value().unwrap() {
            ZValRef::Obj(o) => {
                let got = o.get_property("name").expect_z_str().unwrap().to_bytes();
                assert_eq!(got, b"copilot");
            }
            other => panic!("expect Obj, got {other:?}"),
        }
    }

    {
        let mut arr = ZArray::new();
        arr.insert(InsertKey::NextIndex, ZVal::from("a"));
        let mut val = ZVal::from(arr);

        match val.to_value_mut().unwrap() {
            ZValMut::Arr(a) => {
                a.insert(InsertKey::NextIndex, ZVal::from("b"));
                assert_eq!(a.len(), 2);
            }
            other => panic!("expect Arr, got {other:?}"),
        }
    }

    {
        let mut obj = ZObject::new_by_std_class();
        obj.set_property("foo", ZVal::from("bar"));
        let mut val = ZVal::from(obj);

        match val.to_value_mut().unwrap() {
            ZValMut::Obj(o) => {
                o.set_property("foo", ZVal::from("baz"));
            }
            other => panic!("expect Obj, got {other:?}"),
        }

        let got = val
            .expect_type::<&phper::objects::ZObj>()
            .unwrap()
            .get_property("foo")
            .expect_z_str()
            .unwrap()
            .to_bytes();
        assert_eq!(got, b"baz");
    }

    Ok(())
}

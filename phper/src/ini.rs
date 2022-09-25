// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Apis relate to [crate::sys::zend_ini_entry_def].

use crate::sys::{
    phper_zend_ini_mh, zend_ini_entry_def, OnUpdateBool, OnUpdateLong, OnUpdateReal,
    OnUpdateString, PHP_INI_ALL, PHP_INI_PERDIR, PHP_INI_SYSTEM, PHP_INI_USER,
};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::{
    any::TypeId,
    ffi::CStr,
    mem::{size_of, zeroed},
    os::raw::{c_char, c_void},
    ptr::null_mut,
    str,
    sync::atomic::{AtomicBool, Ordering},
};

static REGISTERED: AtomicBool = AtomicBool::new(false);

static INI_ENTITIES: Lazy<DashMap<String, IniEntity>> = Lazy::new(DashMap::new);

pub struct Ini;

impl Ini {
    pub fn add(name: impl Into<String>, default_value: impl TransformIniValue, policy: Policy) {
        assert!(
            !REGISTERED.load(Ordering::SeqCst),
            "shouldn't add ini after registered"
        );

        let name = name.into();

        INI_ENTITIES.insert(name.clone(), IniEntity::new(name, default_value, policy));
    }

    pub fn get<T: TransformIniValue>(name: &str) -> Option<T> {
        assert!(
            REGISTERED.load(Ordering::SeqCst),
            "shouldn't get ini before registered"
        );

        INI_ENTITIES
            .get(name)
            .and_then(|entity| entity.value().value())
    }

    pub(crate) unsafe fn entries() -> *const zend_ini_entry_def {
        REGISTERED.store(true, Ordering::SeqCst);

        let mut entries = Vec::new();

        for mut entity in INI_ENTITIES.iter_mut() {
            entries.push(entity.value_mut().entry());
        }

        entries.push(zeroed::<zend_ini_entry_def>());

        Box::into_raw(entries.into_boxed_slice()).cast()
    }
}

pub type OnModify = phper_zend_ini_mh;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Policy {
    All = PHP_INI_ALL,
    User = PHP_INI_USER,
    Perdir = PHP_INI_PERDIR,
    System = PHP_INI_SYSTEM,
}

/// The Type which can transform to an ini value.
///
/// Be careful that the size of `arg2` must litter than size of `usize`.
///
/// TODO Add a size compare with usize trait bound, after const generic
/// supports.
pub trait TransformIniValue: Sized + ToString + 'static {
    fn on_modify() -> OnModify;

    /// # Safety
    unsafe fn transform(data: usize) -> Option<Self>;

    fn arg2_type() -> TypeId;

    fn arg2_size() -> usize;

    fn to_text(&self) -> String {
        self.to_string()
    }
}

impl TransformIniValue for bool {
    fn on_modify() -> OnModify {
        Some(OnUpdateBool)
    }

    unsafe fn transform(data: usize) -> Option<Self> {
        Some(data != 0)
    }

    fn arg2_type() -> TypeId {
        TypeId::of::<bool>()
    }

    fn arg2_size() -> usize {
        size_of::<bool>()
    }
}

impl TransformIniValue for i64 {
    fn on_modify() -> OnModify {
        Some(OnUpdateLong)
    }

    unsafe fn transform(data: usize) -> Option<Self> {
        Some(data as i64)
    }

    fn arg2_type() -> TypeId {
        TypeId::of::<i64>()
    }

    fn arg2_size() -> usize {
        size_of::<i64>()
    }
}

impl TransformIniValue for f64 {
    fn on_modify() -> OnModify {
        Some(OnUpdateReal)
    }

    unsafe fn transform(data: usize) -> Option<Self> {
        Some(data as f64)
    }

    fn arg2_type() -> TypeId {
        TypeId::of::<i64>()
    }

    fn arg2_size() -> usize {
        size_of::<i64>()
    }
}

impl TransformIniValue for String {
    fn on_modify() -> OnModify {
        Some(OnUpdateString)
    }

    unsafe fn transform(data: usize) -> Option<Self> {
        let ptr = data as *mut c_char;
        CStr::from_ptr(ptr).to_str().ok().map(|s| s.to_owned())
    }

    fn arg2_type() -> TypeId {
        TypeId::of::<*mut c_char>()
    }

    fn arg2_size() -> usize {
        size_of::<*mut c_char>()
    }
}

pub(crate) struct IniEntity {
    name: String,
    value: usize,
    value_type_id: TypeId,
    default_value: String,
    on_modify: OnModify,
    policy: Policy,
}

impl IniEntity {
    pub(crate) fn new<T: TransformIniValue>(
        name: impl Into<String>, default_value: T, policy: Policy,
    ) -> Self {
        assert!(<T>::arg2_size() <= size_of::<usize>());
        Self {
            name: name.into(),
            value: 0,
            value_type_id: <T>::arg2_type(),
            default_value: default_value.to_text(),
            on_modify: <T>::on_modify(),
            policy,
        }
    }

    pub(crate) fn value<T: TransformIniValue>(&self) -> Option<T> {
        if self.value_type_id != <T>::arg2_type() {
            None
        } else {
            unsafe { <T>::transform(self.value) }
        }
    }

    pub(crate) fn entry(&mut self) -> zend_ini_entry_def {
        create_ini_entry_ex(
            &self.name,
            &self.default_value,
            self.on_modify,
            self.policy as u32,
            &mut self.value as *mut _ as *mut c_void,
        )
    }
}

pub(crate) fn create_ini_entry_ex(
    name: &str, default_value: &str, on_modify: OnModify, modifiable: u32, arg2: *mut c_void,
) -> zend_ini_entry_def {
    #[cfg(any(
        phper_php_version = "8.1",
        phper_php_version = "8.0",
        phper_php_version = "7.4",
        phper_php_version = "7.3",
    ))]
    let (modifiable, name_length) = (modifiable as std::os::raw::c_uchar, name.len() as u16);
    #[cfg(any(
        phper_php_version = "7.2",
        phper_php_version = "7.1",
        phper_php_version = "7.0",
    ))]
    let (modifiable, name_length) = (modifiable as std::os::raw::c_int, name.len() as u32);

    zend_ini_entry_def {
        name: name.as_ptr().cast(),
        on_modify,
        mh_arg1: null_mut(),
        mh_arg2: arg2,
        mh_arg3: null_mut(),
        value: default_value.as_ptr().cast(),
        displayer: None,
        modifiable,
        name_length,
        value_length: default_value.len() as u32,
    }
}

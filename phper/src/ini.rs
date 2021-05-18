//! Apis relate to [crate::sys::zend_ini_entry_def].

use crate::sys::{
    phper_zend_ini_mh, zend_ini_entry_def, OnUpdateBool, OnUpdateLong, OnUpdateReal,
    OnUpdateString, PHP_INI_ALL, PHP_INI_PERDIR, PHP_INI_SYSTEM, PHP_INI_USER,
};
use dashmap::DashMap;
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

thread_local! {
    static INI_ENTITIES: DashMap<String, IniEntity> = DashMap::new();
}

pub struct Ini;

impl Ini {
    pub fn add(name: impl ToString, default_value: impl TransformIniValue, policy: Policy) {
        assert!(
            !REGISTERED.load(Ordering::SeqCst),
            "shouldn't add ini after registered"
        );

        INI_ENTITIES.with(|ini_entities| {
            ini_entities.insert(
                name.to_string(),
                IniEntity::new(name, default_value, policy),
            );
        });
    }

    pub fn get<T: TransformIniValue>(name: &str) -> Option<T> {
        assert!(
            REGISTERED.load(Ordering::SeqCst),
            "shouldn't get ini before registered"
        );

        INI_ENTITIES.with(|ini_entities| {
            ini_entities
                .get(name)
                .and_then(|entity| entity.value().value())
        })
    }

    pub(crate) unsafe fn entries() -> *const zend_ini_entry_def {
        REGISTERED.store(true, Ordering::SeqCst);

        let mut entries = Vec::new();

        INI_ENTITIES.with(|ini_entities| {
            for mut entity in ini_entities.iter_mut() {
                entries.push(entity.value_mut().entry());
            }
        });

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

pub trait TransformIniValue: ToString + 'static {
    fn on_modify(&self) -> OnModify;

    unsafe fn transform(&self, data: usize) -> Option<*mut c_void>;

    fn arg2_type(&self) -> TypeId;

    fn arg2_size(&self) -> usize;

    fn to_text(&self) -> String {
        self.to_string()
    }
}

impl TransformIniValue for bool {
    fn on_modify(&self) -> OnModify {
        Some(OnUpdateBool)
    }

    unsafe fn transform(&self, data: usize) -> Option<*mut c_void> {
        let b = data != 0;
        Some(Box::into_raw(Box::new(b)).cast())
    }

    fn arg2_type(&self) -> TypeId {
        TypeId::of::<bool>()
    }

    fn arg2_size(&self) -> usize {
        size_of::<bool>()
    }
}

impl TransformIniValue for i64 {
    fn on_modify(&self) -> OnModify {
        Some(OnUpdateLong)
    }

    unsafe fn transform(&self, data: usize) -> Option<*mut c_void> {
        let i = data as i64;
        Some(Box::into_raw(Box::new(i)).cast())
    }

    fn arg2_type(&self) -> TypeId {
        TypeId::of::<i64>()
    }

    fn arg2_size(&self) -> usize {
        size_of::<i64>()
    }
}

impl TransformIniValue for f64 {
    fn on_modify(&self) -> OnModify {
        Some(OnUpdateReal)
    }

    unsafe fn transform(&self, data: usize) -> Option<*mut c_void> {
        let f = data as f64;
        Some(Box::into_raw(Box::new(f)).cast())
    }

    fn arg2_type(&self) -> TypeId {
        TypeId::of::<i64>()
    }

    fn arg2_size(&self) -> usize {
        size_of::<i64>()
    }
}

impl TransformIniValue for String {
    fn on_modify(&self) -> OnModify {
        Some(OnUpdateString)
    }

    unsafe fn transform(&self, data: usize) -> Option<*mut c_void> {
        let ptr = data as *mut c_char;
        CStr::from_ptr(ptr)
            .to_str()
            .ok()
            .map(|s| Box::into_raw(Box::new(s.to_owned())).cast())
    }

    fn arg2_type(&self) -> TypeId {
        TypeId::of::<*mut c_char>()
    }

    fn arg2_size(&self) -> usize {
        size_of::<*mut c_char>()
    }
}

pub(crate) struct IniEntity {
    name: String,
    value: usize,
    default_value: String,
    transform: Box<dyn TransformIniValue>,
    policy: Policy,
}

impl IniEntity {
    pub(crate) fn new<T: TransformIniValue>(
        name: impl ToString,
        default_value: T,
        policy: Policy,
    ) -> Self {
        assert!(default_value.arg2_size() <= size_of::<usize>());
        Self {
            name: name.to_string(),
            value: 0,
            default_value: default_value.to_text(),
            transform: Box::new(default_value),
            policy,
        }
    }

    pub(crate) fn value<T: TransformIniValue>(&self) -> Option<T> {
        if self.transform.arg2_type() != TypeId::of::<T>() {
            None
        } else {
            unsafe {
                let ptr = self.transform.transform(self.value);
                ptr.map(|ptr| {
                    let b = Box::from_raw(ptr as *mut T);
                    *b
                })
            }
        }
    }

    pub(crate) fn entry(&mut self) -> zend_ini_entry_def {
        create_ini_entry_ex(
            &self.name,
            &self.default_value,
            self.transform.on_modify(),
            self.policy as u32,
            &mut self.value as *mut _ as *mut c_void,
        )
    }
}

pub(crate) fn create_ini_entry_ex(
    name: &str,
    default_value: &str,
    on_modify: OnModify,
    modifiable: u32,
    arg2: *mut c_void,
) -> zend_ini_entry_def {
    #[cfg(any(
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

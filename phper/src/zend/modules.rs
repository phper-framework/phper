use crate::sys::zend_module_entry;

pub struct ModuleEntry<'a> {
    entry: &'a zend_module_entry,
}


use crate::sys::zend_function_entry;

pub struct FunctionEntries<'a> {
    entries: &'a [zend_function_entry],
}

impl<'a> FunctionEntries<'a> {
    pub const fn from_entries(entries: &'a [zend_function_entry]) -> Self {
        Self { entries }
    }

    #[inline]
    pub fn entries(&self) -> &[zend_function_entry] {
        &self.entries
    }
}

impl<'a> From<&'a FunctionEntries<'a>> for &'a [zend_function_entry] {
    fn from(zfe: &'a FunctionEntries<'a>) -> Self {
        zfe.entries()
    }
}

unsafe impl<'a> Sync for FunctionEntries<'a> {}

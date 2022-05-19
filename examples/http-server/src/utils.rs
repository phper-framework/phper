use std::mem::{replace, take};

pub fn replace_and_set<T: Default>(t: &mut T, f: impl FnOnce(T) -> T) {
    let x = f(take(t));
    let _ = replace(t, x);
}

#[inline]
pub fn replace_and_get<T: Default>(t: &mut T) -> T {
    take(t)
}

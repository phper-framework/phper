use std::mem::{replace, take};

pub fn replace_and_set<T: Default>(t: &mut T, f: impl FnOnce(T) -> T) {
    let x = f(take(t));
    let _ = replace(t, x);
}

pub fn replace_and_get<T: Default, R>(t: &mut T, f: impl FnOnce(T) -> R) -> R {
    f(take(t))
}

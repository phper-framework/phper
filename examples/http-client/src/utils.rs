use std::mem::replace;

pub fn replace_and_set<T>(t: &mut T, init: T, f: impl FnOnce(T) -> T) {
    let x = f(replace(t, init));
    let _ = replace(t, x);
}

pub fn replace_and_get<T, R>(t: &mut T, init: T, f: impl FnOnce(T) -> R) -> R {
    f(replace(t, init))
}

use std::mem::replace;

pub fn replace_and_set<T: Default>(t: &mut T, f: impl FnOnce(T) -> T) {
    let x = f(replace(t, Default::default()));
    let _ = replace(t, x);
}

pub fn replace_and_get<T: Default>(t: &mut T) -> T {
    replace(t, Default::default())
}

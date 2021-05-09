//! Internal useful utils.

pub(crate) fn ensure_end_with_zero(s: impl ToString) -> String {
    let mut s = s.to_string();
    if !s.ends_with('\0') {
        s.push('\0');
    }
    s
}

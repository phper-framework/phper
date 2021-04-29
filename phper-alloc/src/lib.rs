#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]

/// The Box which use php `emalloc` and `efree` to manage memory.
///
/// TODO now feature `allocator_api` is still unstable, using global allocator instead.
pub type EBox<T> = Box<T>;

/// The Vec which use php `emalloc` and `efree` to manage memory.
///
/// TODO now feature `allocator_api` is still unstable, using global allocator instead.
pub type EVec<T> = Vec<T>;

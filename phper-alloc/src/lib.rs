#![warn(rust_2018_idioms, missing_docs)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/112468984?s=200&v=4")]

use std::borrow::Borrow;

/// Duplicate an object without deep copy, but to only add the refcount, for php
/// refcount struct.
pub trait ToRefOwned {
    /// The resulting type after obtaining ownership.
    type Owned: Borrow<Self>;

    /// Creates owned data from borrowed data, by increasing refcount.
    fn to_ref_owned(&mut self) -> Self::Owned;
}

/// Duplicate an object without deep copy, but to only add the refcount, for php
/// refcount struct.
pub trait RefClone {
    /// Returns a refcount value with same reference of the value.
    fn ref_clone(&mut self) -> Self;
}

//! Rust library for supporting probability measures over arbitrary data types.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![warn(missing_docs, unused_import_braces)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod boolean;
pub mod real;
pub mod unit;
// pub mod vector;

mod measurable;

pub use measurable::*;

// #[cfg(feature = "derive")]
// #[doc(hidden)]
// // #[cfg_attr(doc_cfg, doc(cfg(feature = "macros")))]
// pub use mes_derive::Measurable;

#[doc(hidden)]
pub use void;

#[doc(hidden)]
pub use num_traits;

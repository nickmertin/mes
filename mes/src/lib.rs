//! Rust library for supporting probability measures over arbitrary data types.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![warn(missing_docs, unused_import_braces)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod measurable;

pub use measurable::*;

pub mod boolean;
pub mod pair;
pub mod real;
pub mod unit;
pub mod util;
// pub mod vector;

// #[cfg(feature = "derive")]
// #[doc(hidden)]
// // #[cfg_attr(doc_cfg, doc(cfg(feature = "macros")))]
// pub use mes_derive::Measurable;

#[doc(hidden)]
pub use void;

#[doc(hidden)]
pub use num_traits;

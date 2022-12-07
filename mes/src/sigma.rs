//! Provides the [`SigmaAlgebra`] trait.

use with_locals::with;

pub trait SigmaAlgebra<'a> {
    type Space;

    #[with]
    fn empty() -> &'ref Self;

    #[with]
    fn full() -> &'ref Self;

    fn is_empty(&'a self) -> bool;

    #[with]
    fn inversion(&'a self) -> &'ref Self;
}

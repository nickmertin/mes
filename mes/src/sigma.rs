//! Provides the [`SigmaAlgebra`] trait.

pub trait SigmaAlgebra<'a>: 'a {
    type Space;

    fn with_empty<U>(f: impl FnOnce(&'a Self) -> U) -> U;

    fn with_full<U>(f: impl FnOnce(&'a Self) -> U) -> U;

    fn is_empty(&'a self) -> bool;

    fn with_inversion<U>(&'a self, f: impl FnOnce(&Self) -> U) -> U;
}

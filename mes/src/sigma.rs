//! Provides the [`SigmaAlgebra`] trait.

use with_locals::with;

pub trait SigmaAlgebra {
    type Space;

    #[with]
    fn empty() -> &'ref Self;

    #[with]
    fn full() -> &'ref Self;

    fn is_empty(&self) -> bool;

    #[with('local)]
    fn inversion(&self) -> &'local Self;
}

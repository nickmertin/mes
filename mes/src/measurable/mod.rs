use with_locals::with;

mod compose;
mod measure;
mod sigma;

pub use compose::*;
pub use measure::*;
pub use sigma::*;

/// A measurable space.
pub trait Measurable {
    /// The type representing measurable subsets of [`Self`].
    type Subset<'a>: SigmaAlgebra<'a, Space = Self> + ?Sized;

    /// Upcasts a [`Self::Subset`] reference by lifetime. Essentially a proof
    /// that [`Self::Subset`] is covariant with respect to `'a`.
    ///
    /// Inspired by:
    /// <https://internals.rust-lang.org/t/variance-of-lifetime-arguments-in-gats/14769/19>
    ///
    /// The implementation should just be:
    /// ```ignore
    /// fn subset_upcast<'a, 'b: 'a>(s: &'a Self::Subset<'b>) -> &'a Self::Subset<'a> {
    ///     s
    /// }
    /// ```
    ///
    /// If that does not compile, then your definition of [`Self::Subset`] is
    /// probably not covariant.
    fn subset_upcast<'a, 'b: 'a>(s: &'a Self::Subset<'b>) -> &'a Self::Subset<'a>;
}

/// A measurable space for which subsets containing a single point exist for all
/// points in the space.
pub trait PointMeasurable: Measurable {
    #[with]
    /// Computes the subset containing the given point.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```
    /// use mes::PointMeasurable;
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let s: &'ref _ = true.point_subset();
    ///     // `s` contains a reference to the subset containing only `true`.
    /// }
    /// ```
    fn point_subset(&self) -> &'ref Self::Subset<'ref>;
}

/// Describes a measurable function between two measurable spaces.
pub trait MeasurableFn<'subset> {
    /// The domain of the function.
    type Domain: Measurable + ?Sized;

    /// The codomain of the function.
    type Codomain: Measurable + ?Sized;

    #[with]
    /// Computes the preimage of the given subset of the codomain.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```
    /// use mes::{identity, MeasurableFn, PointMeasurable};
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let s: &'ref _ = true.point_subset();
    ///     let s2: &'ref _ = identity::<bool>().preimage(s);
    ///     // `s2` contains a reference to the preimage of `s`.
    /// }
    /// ```
    fn preimage<'a>(
        &'a self,
        s: &'a <Self::Codomain as Measurable>::Subset<'a>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'ref>
    where
        'subset: 'a;
}

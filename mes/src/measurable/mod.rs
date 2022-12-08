use with_locals::with;

mod compose;
mod measure;
mod sigma;

pub use compose::*;
pub use measure::*;
pub use sigma::*;

/// Describes a data type as a measurable space.
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

pub trait PointMeasurable: Measurable {
    #[with]
    fn point_subset(&self) -> &'ref Self::Subset<'ref>;
}

pub trait MeasurableFn<'subset> {
    type Domain: Measurable + ?Sized;

    type Codomain: Measurable + ?Sized;

    #[with]
    fn preimage<'a>(
        &'a self,
        s: &'a <Self::Codomain as Measurable>::Subset<'a>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'ref>
    where
        'subset: 'a;
}

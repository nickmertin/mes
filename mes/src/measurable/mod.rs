use with_locals::with;

mod compose;
mod measure;

pub use compose::*;
pub use measure::*;

use crate::util::{iter::LocalIterator, proxy::Proxy};

/// A measurable space.
pub trait Measurable {
    /// The type representing measurable subsets of [`Self`].
    type Subset<'a>: ?Sized + 'a
    where
        Self: 'a;

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

    #[with]
    /// Computes the empty subset.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```
    /// use mes::Measurable;
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let s: &'ref _ = bool::empty_subset();
    ///     // `s` contains a reference to the empty subset over `bool`.
    ///     assert!(!s.includes_true && !s.includes_false);
    /// }
    /// ```
    fn empty_subset() -> &'ref Self::Subset<'ref>;

    #[with]
    /// Computes the full subset.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```
    /// use mes::Measurable;
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let s: &'ref _ = bool::full_subset();
    ///     // `s` contains a reference to the full subset over `bool`.
    ///     assert!(s.includes_true && s.includes_false);
    /// }
    /// ```
    fn full_subset() -> &'ref Self::Subset<'ref>;

    /// Checks whether the given subset is the empty subset.
    fn subset_is_empty(s: &Self::Subset<'_>) -> bool;

    #[with]
    /// Computes the complement of a subset.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```
    /// use mes::{Measurable, PointMeasurable};
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let s: &'ref _ = true.point_subset();
    ///     let s2: &'ref _ = bool::subset_complement(s);
    ///     // `s2` contains a reference to the complementary subset.
    ///     assert!(!s2.includes_true && s2.includes_false);
    /// }
    /// ```
    fn subset_complement(s: &Self::Subset<'_>) -> &'ref Self::Subset<'ref>;

    #[with]
    /// Computes the union of a finite collection of subsets.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```
    /// use mes::{Measurable, PointMeasurable};
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let s1: &'ref _ = true.point_subset();
    ///     let s2: &'ref _ = false.point_subset();
    ///     let s: &'ref _ =
    ///         bool::subset_union([bool::subset_upcast(s1), bool::subset_upcast(s2)].into_iter());
    ///     // `s` contains a reference to the subset union.
    ///     assert!(s.includes_true && s.includes_false);
    /// }
    /// ```
    fn subset_union<'a>(
        subsets: impl Iterator<Item = &'a Proxy<'a, Self::Subset<'a>>> + Clone + 'a,
    ) -> &'ref Self::Subset<'ref>
    where
        Self: 'a;

    #[with]
    /// Computes the intersection of a finite collection of subsets.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```no_run
    /// use mes::{Measurable, PointMeasurable};
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let s1: &'ref _ = true.point_subset();
    ///     let s2: &'ref _ = false.point_subset();
    ///     let s: &'ref _ =
    ///         bool::subset_intersect([bool::subset_upcast(s1), bool::subset_upcast(s2)].into_iter());
    ///     // `s` contains a reference to the subset intersection.
    /// }
    /// ```
    fn subset_intersect<'a>(
        subsets: impl Iterator<Item = &'a Self::Subset<'a>> + Clone + 'a,
    ) -> &'ref Self::Subset<'ref>
    where
        Self: 'a,
    {
        // let result: &'ref _ = Self::subset_union(subsets.m)
        todo!()
    }
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
    fn point_subset<'a>(&'a self) -> &'ref Self::Subset<'ref>;
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
    ///     assert!(s2.includes_true && !s2.includes_false);
    /// }
    /// ```
    fn preimage<'a>(
        &'a self,
        s: &'a <Self::Codomain as Measurable>::Subset<'a>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'ref>
    where
        'subset: 'a;
}

use with_locals::with;

/// A [σ-algebra](https://en.wikipedia.org/wiki/%CE%A3-algebra) over a given
/// space.
pub trait SigmaAlgebra<'a> {
    /// The space over which the σ-algebra is defined.
    type Space: ?Sized;

    #[with]
    /// Computes the empty subset.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```
    /// use mes::{Measurable, SigmaAlgebra};
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let s: &'ref _ = <bool as Measurable>::Subset::empty();
    ///     // `s` contains a reference to the empty subset over `bool`.
    /// }
    /// ```
    fn empty() -> &'ref Self;

    #[with]
    /// Computes the full subset.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```
    /// use mes::{Measurable, SigmaAlgebra};
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let s: &'ref _ = <bool as Measurable>::Subset::full();
    ///     // `s` contains a reference to the full subset over `bool`.
    /// }
    /// ```
    fn full() -> &'ref Self;

    /// Checks whether the given subset is the empty subset.
    fn is_empty(&'a self) -> bool;

    #[with]
    /// Computes the complement of a subset.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```
    /// use mes::{PointMeasurable, SigmaAlgebra};
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let s: &'ref _ = true.point_subset();
    ///     let s2: &'ref _ = s.complement();
    ///     // `s2` contains a reference to the complementary subset.
    /// }
    /// ```
    fn complement(&'a self) -> &'ref Self;
}

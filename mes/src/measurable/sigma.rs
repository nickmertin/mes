use with_locals::with;

/// Describes a [σ-algebra](https://en.wikipedia.org/wiki/%CE%A3-algebra) over a
/// given space.
pub trait SigmaAlgebra<'a> {
    /// The space over which the σ-algebra is defined.
    type Space: ?Sized;

    #[with]
    fn empty() -> &'ref Self;

    #[with]
    fn full() -> &'ref Self;

    fn is_empty(&'a self) -> bool;

    #[with]
    fn inversion(&'a self) -> &'ref Self;
}

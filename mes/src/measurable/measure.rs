use core::ops::{Mul, MulAssign};
use with_locals::with;

use crate::{measurable::Measurable, real::Real};

/// Describes a measure on a measurable space.
pub trait Measure<'subset>:
    From<Self::PMeasure> + Mul<Self::R, Output = Self> + MulAssign<Self::R>
{
    /// The type of real number used in the measure.
    type R: Real;

    /// The measurable space.
    type Space: Measurable + ?Sized;

    /// The type of measurements.
    type Measurement;

    /// The type of analogous probability measures.
    type PMeasure;

    #[with]
    /// Computes the measure of the given subset.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```
    /// use mes::{boolean::BoolMeasure, Measure, PointMeasurable};
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let s: &'ref _ = true.point_subset();
    ///     let m = BoolMeasure {
    ///         true_value: 1f32,
    ///         false_value: 0f32,
    ///     };
    ///     let x: &'ref _ = m.measure(s);
    ///     // `x` contains a reference to the measurement of the set containing `true`.
    /// }
    /// ```
    fn measure<'a>(
        &'a self,
        domain: &'a <Self::Space as Measurable>::Subset<'a>,
    ) -> &'ref Self::Measurement
    where
        'subset: 'a;

    /// Attempts to normalize the measure into a probability measure.
    ///
    /// Returns [`None`] if that is impossible, such as if the measure has zero
    /// weight.
    fn normalize(&self) -> Option<Self::PMeasure>;
}

/// Describes a measure for which point measurements can be made.
///
/// Generally this will either be a measure over a discrete measurable space, in
/// which case point measurements are equivalent to subset measurements over the
/// corresponding point subset (where it exists), or a measure whose derivative
/// with respect to the space can be considered.
pub trait PointMeasure<'subset>: Measure<'subset> {
    /// The type of point measurements.
    type PointMeasurement;

    #[with]
    /// Computes the point measure at a given point.
    ///
    /// This function uses continuation-passing style (CPS) via [`with_locals`].
    /// Use it like so:
    /// ```
    /// use mes::{boolean::BoolMeasure, PointMeasure};
    /// use with_locals::with;
    ///
    /// #[with]
    /// fn main() {
    ///     let m = BoolMeasure {
    ///         true_value: 1f32,
    ///         false_value: 0f32,
    ///     };
    ///     let x: &'ref _ = m.measure_at(&true);
    ///     // `x` contains a reference to the point measurement at `true`.
    /// }
    /// ```
    fn measure_at(&self, value: &Self::Space) -> &'ref Self::PointMeasurement;
}

/// Describes a type of measure which is capable of representing Dirac measures.
pub trait DiracMeasure<'subset>: Measure<'subset> {
    /// Constructs a Dirac measure at the given point.
    fn dirac(point: &Self::Space) -> Self;
}

//! Implementation of the unit type as a measurable space.

use derive_more::{Add, AddAssign, Mul, MulAssign};
use type_variance::Contravariant;
use with_locals::with;

use crate::{
    real::Real, DiracMeasure, Measurable, MeasurableFn, Measure, PointMeasurable, PointMeasure,
};

/// A subset of the unit type.
pub struct UnitSubset {
    /// Whether the subset is the full or empty subset.
    pub full: bool,
}

/// A function whose codomain is the unit type.
pub struct UnitFunction<T: ?Sized>(Contravariant<T>);

impl<'subset, T: Measurable + ?Sized> MeasurableFn<'subset> for UnitFunction<T> {
    type Domain = T;

    type Codomain = ();

    #[with]
    fn preimage<'a>(
        &'a self,
        s: &'a <Self::Codomain as Measurable>::Subset<'a>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'ref>
    where
        'subset: 'a,
    {
        if s.full {
            let x: &'ref _ = T::full_subset();
            x
        } else {
            let x: &'ref _ = T::empty_subset();
            x
        }
    }
}

impl Measurable for () {
    type Subset<'a> = UnitSubset;

    fn subset_upcast<'a, 'b: 'a>(s: &'a Self::Subset<'b>) -> &'a Self::Subset<'a> {
        s
    }

    #[with]
    fn empty_subset() -> &'ref Self::Subset<'ref> {
        &UnitSubset { full: false }
    }

    #[with]
    fn full_subset() -> &'ref Self::Subset<'ref> {
        &UnitSubset { full: true }
    }

    fn subset_is_empty(s: &Self::Subset<'_>) -> bool {
        !s.full
    }

    #[with]
    fn subset_complement(s: &Self::Subset<'_>) -> &'ref Self::Subset<'ref> {
        &UnitSubset { full: !s.full }
    }

    #[with]
    fn subset_union<'a>(
        mut subsets: impl Iterator<Item = &'a Self::Subset<'a>> + Clone + 'a,
    ) -> &'ref Self::Subset<'ref>
    where
        Self: 'a,
    {
        &UnitSubset {
            full: subsets.any(|s| s.full),
        }
    }
}

impl PointMeasurable for () {
    #[with]
    fn point_subset(&self) -> &'ref Self::Subset<'ref> {
        #[with]
        let x = Self::full_subset();
        x
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Add, AddAssign, Mul, MulAssign)]
/// A measure on the unit type.
pub struct UnitMeasure<R: Real> {
    /// The weight of the measure.
    pub weight: R,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A probability measure on the unit type.
pub struct UnitPMeasure;

impl<R: Real> From<UnitPMeasure> for UnitMeasure<R> {
    fn from(_: UnitPMeasure) -> Self {
        Self { weight: R::one() }
    }
}

impl<'subset, R: Real> Measure<'subset> for UnitMeasure<R> {
    type R = R;

    type Space = ();

    type Measurement = R;

    type PMeasure = UnitPMeasure;

    #[with]
    fn measure<'a>(
        &'a self,
        domain: &'a <Self::Space as Measurable>::Subset<'a>,
    ) -> &'ref Self::Measurement
    where
        'subset: 'a,
    {
        &if domain.full { self.weight } else { R::zero() }
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        R::normalize_static([self.weight])?;
        Some(UnitPMeasure)
    }
}

impl<'subset, R: Real> PointMeasure<'subset> for UnitMeasure<R> {
    type PointMeasurement = R;

    #[with]
    fn measure_at(&self, _value: &Self::Space) -> &'ref Self::PointMeasurement {
        &self.weight
    }
}

impl<'subset, R: Real> DiracMeasure<'subset> for UnitMeasure<R> {
    fn dirac(_point: &Self::Space) -> Self {
        Self { weight: R::one() }
    }
}

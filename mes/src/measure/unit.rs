use derive_more::{Add, AddAssign, Mul, MulAssign};
use with_locals::with;

use crate::{measurable::Measurable, real::Real};

use super::{DiracMeasure, Measure, PointMeasure};

#[derive(Clone, Copy, PartialEq, PartialOrd, Add, AddAssign, Mul, MulAssign)]
struct UnitMeasure<R: Real> {
    weight: R,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct UnitPMeasure;

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
    fn point(_value: &Self::Space) -> Self {
        Self { weight: R::one() }
    }
}

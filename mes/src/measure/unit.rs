use derive_more::{Add, AddAssign, Mul, MulAssign};
use with_locals::with;

use crate::{measurable::Measurable, real::Real};

use super::{DiracMeasure, Measure};

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

impl<R: Real> Measure for UnitMeasure<R> {
    type R = R;

    type Space = ();

    type Measurement = R;

    type PointMeasurement = R;

    type PMeasure = UnitPMeasure;

    #[with]
    fn measure(&self, domain: &<Self::Space as Measurable>::Subset) -> &'ref Self::Measurement {
        &if domain.full { self.weight } else { R::zero() }
    }

    fn measure_at(&self, _value: &Self::Space) -> Self::PointMeasurement {
        self.weight
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        R::normalize_static([self.weight])?;
        Some(UnitPMeasure)
    }
}

impl<R: Real> DiracMeasure for UnitMeasure<R> {
    fn point(_value: &Self::Space) -> Self {
        Self { weight: R::one() }
    }
}

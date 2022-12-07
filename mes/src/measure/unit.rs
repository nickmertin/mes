use derive_more::{Add, AddAssign, Mul, MulAssign};

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

impl<'a, R: Real> Measure<'a> for UnitMeasure<R> {
    type R = R;

    type Space = ();

    type Measurement = R;

    type PointMeasurement = R;

    type PMeasure = UnitPMeasure;

    fn with_measure<'subset: 'a, 'b, U>(
        &'a self,
        domain: &'b <Self::Space as Measurable>::Subset<'subset>,
        f: impl FnOnce(&Self::Measurement) -> U + 'b,
    ) -> U {
        f(&if domain.full { self.weight } else { R::zero() })
    }

    fn measure_at(&self, _value: &Self::Space) -> Self::PointMeasurement {
        self.weight
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        R::normalize_static([self.weight])?;
        Some(UnitPMeasure)
    }
}

impl<'a, R: Real> DiracMeasure<'a> for UnitMeasure<R> {
    fn point(_value: &Self::Space) -> Self {
        Self { weight: R::one() }
    }
}

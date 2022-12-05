use derive_more::{Add, AddAssign, Mul, MulAssign};

use crate::real::Real;

use super::{DiracMeasure, Measure};

#[derive(Clone, Copy, PartialEq, PartialOrd, Add, AddAssign, Mul, MulAssign)]
struct UnitMeasure<R: Real>(R);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct UnitPMeasure;

impl<R: Real> From<UnitPMeasure> for UnitMeasure<R> {
    fn from(_: UnitPMeasure) -> Self {
        Self(R::one())
    }
}

impl<R: Real> Measure for UnitMeasure<R> {
    type R = R;

    type Space = ();

    type Measurement<'a> = R
    where
        Self: 'a;

    type PMeasure = UnitPMeasure;

    fn measure_at(&self, _value: &Self::Space) -> Self::Measurement<'_> {
        self.0
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        R::normalize_static([self.0])?;
        Some(UnitPMeasure)
    }
}

impl<R: Real> DiracMeasure for UnitMeasure<R> {
    fn point(_value: &Self::Space) -> Self {
        Self(R::one())
    }
}

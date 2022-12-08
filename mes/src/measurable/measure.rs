use core::ops::{Mul, MulAssign};
use with_locals::with;

use crate::{measurable::Measurable, real::Real};

pub trait Measure<'subset>:
    From<Self::PMeasure> + Mul<Self::R, Output = Self> + MulAssign<Self::R>
{
    type R: Real;

    type Space: Measurable + ?Sized;

    type Measurement;

    type PMeasure;

    #[with]
    fn measure<'a>(
        &'a self,
        domain: &'a <Self::Space as Measurable>::Subset<'a>,
    ) -> &'ref Self::Measurement
    where
        'subset: 'a;

    fn normalize(&self) -> Option<Self::PMeasure>;
}

pub trait PointMeasure<'subset>: Measure<'subset> {
    type PointMeasurement;

    #[with]
    fn measure_at(&self, value: &Self::Space) -> &'ref Self::PointMeasurement;
}

pub trait DiracMeasure<'subset>: Measure<'subset> {
    fn point(value: &Self::Space) -> Self;
}

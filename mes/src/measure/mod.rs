//! Measure trait.

use core::ops::{Mul, MulAssign};

use crate::real::Real;

pub mod boolean;
pub mod unit;

pub trait Measure:
    PartialEq + From<Self::PMeasure> + Mul<Self::R, Output = Self> + MulAssign<Self::R>
{
    type R: Real;

    type Space;

    type Measurement<'a>
    where
        Self: 'a;

    type PMeasure;

    fn measure_at(&self, value: &Self::Space) -> Self::Measurement<'_>;

    fn normalize(&self) -> Option<Self::PMeasure>;
}

pub trait DiracMeasure: Measure {
    fn point(value: &Self::Space) -> Self;
}

pub trait RecursiveMeasure: Measure {
    type RMeasure: RecursiveMeasure<R = Self::R, Space = Self> + From<Self::RPMeasure>;

    type RPMeasure: RecursiveMeasure<R = Self::R, Space = Self::PMeasure>;
}

pub trait RecursiveMeasureExt: RecursiveMeasure {
    fn join(&self) -> Self::Space
    where
        Self::Space: RecursiveMeasure<RMeasure = Self>;
}

impl<M: RecursiveMeasure> RecursiveMeasureExt for M {
    fn join(&self) -> Self::Space
    where
        Self::Space: RecursiveMeasure<RMeasure = Self>,
    {
        todo!()
    }
}

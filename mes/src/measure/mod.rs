//! Measure trait.

use core::ops::{Mul, MulAssign};

use crate::{
    measurable::{Measurable, MeasurableFn},
    real::Real,
};

pub mod boolean;
pub mod unit;

pub trait Measure: From<Self::PMeasure> + Mul<Self::R, Output = Self> + MulAssign<Self::R> {
    type R: Real;

    type Space: Measurable + ?Sized;

    type Measurement<'a>
    where
        Self: 'a;

    type PointMeasurement<'a>
    where
        Self: 'a;

    type PMeasure;

    fn measure<'a>(
        &'a self,
        domain: &'a <Self::Space as Measurable>::Subset<'a>,
    ) -> Self::Measurement<'a>;

    fn measure_at(&self, value: &Self::Space) -> Self::PointMeasurement<'_>;

    fn normalize(&self) -> Option<Self::PMeasure>;
}

pub trait DiracMeasure: Measure {
    fn point(value: &Self::Space) -> Self;
}

pub struct CompositeMeasure<
    'a,
    // T: Measurable + ?Sized + 'a,
    // U: Measurable + ?Sized,
    F: MeasurableFn<'a> + ?Sized,
    M: Measure<Space = F::Domain>,
> {
    function: &'a F,
    measure: M,
}

pub struct CompositePMeasure<
    'a,
    // T: Measurable + ?Sized + 'a,
    // U: Measurable + ?Sized,
    F: MeasurableFn<'a> + ?Sized,
    M: Measure<Space = F::Domain>,
> {
    function: &'a F,
    measure: M::PMeasure,
}

impl<'a, F: MeasurableFn<'a> + ?Sized, M: Measure<Space = F::Domain>>
    From<CompositePMeasure<'a, F, M>> for CompositeMeasure<'a, F, M>
{
    fn from(p: CompositePMeasure<'a, F, M>) -> Self {
        Self {
            function: p.function,
            measure: p.measure.into(),
        }
    }
}

impl<'a, F: MeasurableFn<'a> + ?Sized, M: Measure<Space = F::Domain>> Mul<M::R>
    for CompositeMeasure<'a, F, M>
{
    type Output = Self;

    fn mul(self, rhs: M::R) -> Self::Output {
        Self {
            function: self.function,
            measure: self.measure * rhs,
        }
    }
}

impl<'a, F: MeasurableFn<'a> + ?Sized, M: Measure<Space = F::Domain>> MulAssign<M::R>
    for CompositeMeasure<'a, F, M>
{
    fn mul_assign(&mut self, rhs: M::R) {
        self.measure *= rhs
    }
}

impl<'a, F: MeasurableFn<'a> + ?Sized, M: Measure<Space = F::Domain>> Measure
    for CompositeMeasure<'a, F, M>
{
    type R = M::R;

    type Space = F::Codomain;

    type Measurement<'b> = M::Measurement<'b>
    where
        Self: 'b;

    type PointMeasurement<'b> = M::PointMeasurement<'b>
    where
        Self: 'b;

    type PMeasure = CompositePMeasure<'a, F, M>;

    fn measure<'b>(
        &'b self,
        domain: &'b <Self::Space as Measurable>::Subset<'b>,
    ) -> Self::Measurement<'b> {
        // F::with_preimage(self.function, domain, |s| self.measure.measure(s))
        todo!()
    }

    fn measure_at(&self, value: &Self::Space) -> Self::PointMeasurement<'_> {
        todo!()
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        Some(CompositePMeasure {
            function: self.function,
            measure: self.measure.normalize()?,
        })
    }
}

pub fn compose<'a, F: MeasurableFn<'a> + ?Sized, M: Measure<Space = F::Domain>>(
    function: &'a F,
    measure: M,
) -> CompositeMeasure<'a, F, M> {
    CompositeMeasure { function, measure }
}

// pub trait RecursiveMeasure: Measure {
//     type RMeasure: RecursiveMeasure<R = Self::R, Space = Self> +
// From<Self::RPMeasure>;

//     type RPMeasure: RecursiveMeasure<R = Self::R, Space = Self::PMeasure>;
// }

// pub trait RecursiveMeasureExt: RecursiveMeasure {
//     fn join(&self) -> Self::Space
//     where
//         Self::Space: RecursiveMeasure<RMeasure = Self>;
// }

// impl<M: RecursiveMeasure> RecursiveMeasureExt for M {
//     fn join(&self) -> Self::Space
//     where
//         Self::Space: RecursiveMeasure<RMeasure = Self>,
//     {
//         todo!()
//     }
// }

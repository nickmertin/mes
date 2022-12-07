//! Measure trait.

use core::{
    marker::PhantomData,
    ops::{Mul, MulAssign},
};

use with_locals::with;

use crate::{
    measurable::{Measurable, MeasurableFn},
    real::Real,
};

pub mod boolean;
pub mod unit;

pub trait Measure: From<Self::PMeasure> + Mul<Self::R, Output = Self> + MulAssign<Self::R> {
    type R: Real;

    type Space: Measurable + ?Sized;

    type Measurement;

    type PointMeasurement;

    type PMeasure;

    #[with]
    fn measure(&self, domain: &<Self::Space as Measurable>::Subset) -> &'ref Self::Measurement;

    fn measure_at(&self, value: &Self::Space) -> Self::PointMeasurement;

    fn normalize(&self) -> Option<Self::PMeasure>;
}

pub trait DiracMeasure: Measure {
    fn point(value: &Self::Space) -> Self;
}

pub struct CompositeMeasure<
    'a,
    // T: Measurable + ?Sized + 'a,
    // U: Measurable + ?Sized,
    F: MeasurableFn + ?Sized,
    M: Measure<Space = F::Domain>,
> {
    function: &'a F,
    measure: M,
}

pub struct CompositePMeasure<
    'a,
    // T: Measurable + ?Sized + 'a,
    // U: Measurable + ?Sized,
    F: MeasurableFn + ?Sized,
    M: Measure<Space = F::Domain>,
> {
    function: &'a F,
    measure: M::PMeasure,
}

impl<'a, F: MeasurableFn + ?Sized, M: Measure<Space = F::Domain>> From<CompositePMeasure<'a, F, M>>
    for CompositeMeasure<'a, F, M>
{
    fn from(p: CompositePMeasure<'a, F, M>) -> Self {
        Self {
            function: p.function,
            measure: p.measure.into(),
        }
    }
}

impl<'a, F: MeasurableFn + ?Sized, M: Measure<Space = F::Domain>> Mul<M::R>
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

impl<'a, F: MeasurableFn + ?Sized, M: Measure<Space = F::Domain>> MulAssign<M::R>
    for CompositeMeasure<'a, F, M>
{
    fn mul_assign(&mut self, rhs: M::R) {
        self.measure *= rhs
    }
}

impl<'a, F: MeasurableFn + ?Sized, M: Measure<Space = F::Domain>> Measure
    for CompositeMeasure<'a, F, M>
{
    type R = M::R;

    type Space = F::Codomain;

    type Measurement = M::Measurement;

    type PointMeasurement = M::PointMeasurement;

    type PMeasure = CompositePMeasure<'a, F, M>;

    #[with]
    fn measure(&self, domain: &<Self::Space as Measurable>::Subset) -> &'ref Self::Measurement {
        // let g: &'a F = &self.function;
        // let m: &'c M = &self.measure;
        // <F as MeasurableFn<'a>>::with_preimage::<'a, 'c, _>(g, domain, |s| {
        //     // m.with_measure::<'c, _>(s, |x| {
        //     //     todo!()
        //     //     // f(x)
        //     // })
        //     todo!()
        // })
        #[with]
        let s = <F as MeasurableFn>::preimage(&self.function, domain);
        #[with]
        let x = self.measure.measure(s);
        &x
        // todo!()
    }

    fn measure_at(&self, value: &Self::Space) -> Self::PointMeasurement {
        todo!()
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        Some(CompositePMeasure {
            function: self.function,
            measure: self.measure.normalize()?,
        })
    }
}

pub fn compose<F: MeasurableFn + ?Sized, M: Measure<Space = F::Domain>>(
    function: &F,
    measure: M,
) -> CompositeMeasure<'_, F, M> {
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

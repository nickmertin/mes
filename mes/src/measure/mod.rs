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

pub trait Measure<'subset>:
    From<Self::PMeasure> + Mul<Self::R, Output = Self> + MulAssign<Self::R>
{
    type R: Real;

    type Space: Measurable + ?Sized;

    type Measurement;

    type PointMeasurement;

    type PMeasure;

    #[with]
    fn measure<'a>(
        &'a self,
        domain: &'a <Self::Space as Measurable>::Subset<'a>,
    ) -> &'ref Self::Measurement
    where
        'subset: 'a;

    fn measure_at(&self, value: &Self::Space) -> Self::PointMeasurement;

    fn normalize(&self) -> Option<Self::PMeasure>;
}

pub trait DiracMeasure<'subset>: Measure<'subset> {
    fn point(value: &Self::Space) -> Self;
}

pub struct CompositeMeasure<
    'subset,
    // T: Measurable + ?Sized + 'a,
    // U: Measurable + ?Sized,
    F: MeasurableFn<'subset> + ?Sized,
    M: Measure<'subset, Space = F::Domain>,
> {
    function: &'subset F,
    measure: M,
}

pub struct CompositePMeasure<
    'subset,
    // T: Measurable + ?Sized + 'a,
    // U: Measurable + ?Sized,
    F: MeasurableFn<'subset> + ?Sized,
    M: Measure<'subset, Space = F::Domain>,
> {
    function: &'subset F,
    measure: M::PMeasure,
}

impl<'subset, F: MeasurableFn<'subset> + ?Sized, M: Measure<'subset, Space = F::Domain>>
    From<CompositePMeasure<'subset, F, M>> for CompositeMeasure<'subset, F, M>
{
    fn from(p: CompositePMeasure<'subset, F, M>) -> Self {
        Self {
            function: p.function,
            measure: p.measure.into(),
        }
    }
}

impl<'subset, F: MeasurableFn<'subset> + ?Sized, M: Measure<'subset, Space = F::Domain>> Mul<M::R>
    for CompositeMeasure<'subset, F, M>
{
    type Output = Self;

    fn mul(self, rhs: M::R) -> Self::Output {
        Self {
            function: self.function,
            measure: self.measure * rhs,
        }
    }
}

impl<'subset, F: MeasurableFn<'subset> + ?Sized, M: Measure<'subset, Space = F::Domain>>
    MulAssign<M::R> for CompositeMeasure<'subset, F, M>
{
    fn mul_assign(&mut self, rhs: M::R) {
        self.measure *= rhs
    }
}

impl<'subset, F: MeasurableFn<'subset> + ?Sized, M: Measure<'subset, Space = F::Domain>>
    Measure<'subset> for CompositeMeasure<'subset, F, M>
{
    type R = M::R;

    type Space = F::Codomain;

    type Measurement = M::Measurement;

    type PointMeasurement = M::PointMeasurement;

    type PMeasure = CompositePMeasure<'subset, F, M>;

    #[with]
    fn measure<'a>(
        &'a self,
        domain: &'a <Self::Space as Measurable>::Subset<'a>,
    ) -> &'ref Self::Measurement
    where
        'subset: 'a,
    {
        // let g: &'a F = &self.function;
        // let m: &'c M = &self.measure;
        // <F as MeasurableFn<'a>>::with_preimage::<'a, 'c, _>(g, domain, |s| {
        //     // m.with_measure::<'c, _>(s, |x| {
        //     //     todo!()
        //     //     // f(x)
        //     // })
        //     todo!()
        // })
        // let d = <F as MeasurableFn>::Codomain::subset_upcast::<'a, 'subset>(domain);
        #[with]
        let s: &<<F as MeasurableFn>::Domain as Measurable>::Subset<'a> =
            <F as MeasurableFn>::preimage(&self.function, domain);
        #[with]
        let x = self
            .measure
            .measure(<F as MeasurableFn>::Domain::subset_upcast(s));
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

pub fn compose<'a, F: MeasurableFn<'a> + ?Sized, M: Measure<'a, Space = F::Domain>>(
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

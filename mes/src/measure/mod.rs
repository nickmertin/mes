//! Measure trait.

use core::{
    marker::PhantomData,
    ops::{Mul, MulAssign},
};

use crate::{
    measurable::{Measurable, MeasurableFn},
    real::Real,
};

pub mod boolean;
pub mod unit;

pub trait Measure<'a>:
    From<Self::PMeasure> + Mul<Self::R, Output = Self> + MulAssign<Self::R>
{
    type R: Real;

    type Space: Measurable + ?Sized;

    type Measurement;

    type PointMeasurement;

    type PMeasure;

    fn with_measure<'subset: 'a, 'b, U>(
        &'a self,
        domain: &'b <Self::Space as Measurable>::Subset<'subset>,
        f: impl FnOnce(&Self::Measurement) -> U + 'b,
    ) -> U
    where
        'a: 'b;

    fn measure_at(&self, value: &Self::Space) -> Self::PointMeasurement;

    fn normalize(&self) -> Option<Self::PMeasure>;
}

pub trait DiracMeasure<'a>: Measure<'a> {
    fn point(value: &Self::Space) -> Self;
}

pub struct CompositeMeasure<
    'a,
    'b: 'a,
    // T: Measurable + ?Sized + 'a,
    // U: Measurable + ?Sized,
    F: MeasurableFn<'b> + ?Sized,
    M: Measure<'b, Space = F::Domain>,
> {
    function: &'a F,
    measure: M,
    _phantom: PhantomData<&'b ()>,
}

pub struct CompositePMeasure<
    'a,
    'b: 'a,
    // T: Measurable + ?Sized + 'a,
    // U: Measurable + ?Sized,
    F: MeasurableFn<'b> + ?Sized,
    M: Measure<'b, Space = F::Domain>,
> {
    function: &'a F,
    measure: M::PMeasure,
    _phantom: PhantomData<&'b ()>,
}

impl<'a, 'b: 'a, F: MeasurableFn<'b> + ?Sized, M: Measure<'b, Space = F::Domain>>
    From<CompositePMeasure<'a, 'b, F, M>> for CompositeMeasure<'a, 'b, F, M>
{
    fn from(p: CompositePMeasure<'a, 'b, F, M>) -> Self {
        Self {
            function: p.function,
            measure: p.measure.into(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, 'b: 'a, F: MeasurableFn<'b> + ?Sized, M: Measure<'b, Space = F::Domain>> Mul<M::R>
    for CompositeMeasure<'a, 'b, F, M>
{
    type Output = Self;

    fn mul(self, rhs: M::R) -> Self::Output {
        Self {
            function: self.function,
            measure: self.measure * rhs,
            _phantom: PhantomData,
        }
    }
}

impl<'a, 'b: 'a, F: MeasurableFn<'b> + ?Sized, M: Measure<'b, Space = F::Domain>> MulAssign<M::R>
    for CompositeMeasure<'a, 'b, F, M>
{
    fn mul_assign(&mut self, rhs: M::R) {
        self.measure *= rhs
    }
}

impl<'a, F: MeasurableFn<'a> + ?Sized, M: Measure<'a, Space = F::Domain>> Measure<'a>
    for CompositeMeasure<'a, 'a, F, M>
{
    type R = M::R;

    type Space = F::Codomain;

    type Measurement = M::Measurement;

    type PointMeasurement = M::PointMeasurement;

    type PMeasure = CompositePMeasure<'a, 'a, F, M>;

    fn with_measure<'subset: 'a, 'c, U>(
        &'a self,
        domain: &'c <Self::Space as Measurable>::Subset<'subset>,
        f: impl FnOnce(&Self::Measurement) -> U + 'c,
    ) -> U
    where
        'a: 'c,
    {
        let g: &'a F = &self.function;
        let m: &'c M = &self.measure;
        // <F as MeasurableFn<'a>>::with_preimage::<'a, 'c, _>(g, domain, |s| {
        //     // m.with_measure::<'c, _>(s, |x| {
        //     //     todo!()
        //     //     // f(x)
        //     // })
        //     todo!()
        // })
        todo!()
    }

    fn measure_at(&self, value: &Self::Space) -> Self::PointMeasurement {
        todo!()
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        Some(CompositePMeasure {
            function: self.function,
            measure: self.measure.normalize()?,
            _phantom: PhantomData,
        })
    }
}

pub fn compose<'a, 'b: 'a, F: MeasurableFn<'b> + ?Sized, M: Measure<'b, Space = F::Domain>>(
    function: &'a F,
    measure: M,
) -> CompositeMeasure<'a, 'b, F, M> {
    CompositeMeasure {
        function,
        measure,
        _phantom: PhantomData,
    }
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

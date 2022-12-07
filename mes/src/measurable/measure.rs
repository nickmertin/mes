use core::ops::{Mul, MulAssign};
use with_locals::with;

use crate::{
    measurable::{Measurable, MeasurableFn, PointMeasurable},
    real::Real,
};

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

    type PMeasure = CompositePMeasure<'subset, F, M>;

    #[with]
    fn measure<'a>(
        &'a self,
        domain: &'a <Self::Space as Measurable>::Subset<'a>,
    ) -> &'ref Self::Measurement
    where
        'subset: 'a,
    {
        #[with]
        let s = <F as MeasurableFn>::preimage(&self.function, domain);

        #[with]
        let x = self
            .measure
            .measure(<F as MeasurableFn>::Domain::subset_upcast(s));

        x
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        Some(CompositePMeasure {
            function: self.function,
            measure: self.measure.normalize()?,
        })
    }
}

impl<'subset, F: MeasurableFn<'subset> + ?Sized, M: Measure<'subset, Space = F::Domain>>
    PointMeasure<'subset> for CompositeMeasure<'subset, F, M>
where
    <F as MeasurableFn<'subset>>::Codomain: PointMeasurable,
{
    type PointMeasurement = M::Measurement;

    #[with]
    fn measure_at<'a>(&'a self, value: &'a Self::Space) -> &'ref Self::PointMeasurement {
        // TODO: make this be able to take point measurements from M as well, depending
        // on the type of function.

        #[with]
        let domain = value.point_subset();

        #[with]
        let s = <F as MeasurableFn>::preimage(
            &self.function,
            <F as MeasurableFn>::Codomain::subset_upcast(domain),
        );

        #[with]
        let x = self
            .measure
            .measure(<F as MeasurableFn>::Domain::subset_upcast(s));

        x
    }
}

pub fn compose<'a, F: MeasurableFn<'a> + ?Sized, M: Measure<'a, Space = F::Domain>>(
    function: &'a F,
    measure: M,
) -> CompositeMeasure<'a, F, M> {
    CompositeMeasure { function, measure }
}

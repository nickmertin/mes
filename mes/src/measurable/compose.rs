use core::ops::{Mul, MulAssign};
use with_locals::with;

use crate::{
    measurable::{Measurable, MeasurableFn, PointMeasurable},
    Measure, PointMeasure,
};

/// A measure which is the composition of a function with another measure.
pub struct CompositeMeasure<
    'subset,
    F: MeasurableFn<'subset> + ?Sized,
    M: Measure<'subset, Space = F::Domain>,
> {
    function: &'subset F,
    measure: M,
}

/// A probability measure which is the composition of a function with another
/// probability measure.
pub struct CompositePMeasure<
    'subset,
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
        let s = <F as MeasurableFn>::preimage(self.function, domain);

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
            self.function,
            <F as MeasurableFn>::Codomain::subset_upcast(domain),
        );

        #[with]
        let x = self
            .measure
            .measure(<F as MeasurableFn>::Domain::subset_upcast(s));

        x
    }
}

/// Composes a function with a measure. The domain of the function should be the
/// measurable space associated with the measure.
pub fn compose_measure<'a, F: MeasurableFn<'a> + ?Sized, M: Measure<'a, Space = F::Domain>>(
    function: &'a F,
    measure: M,
) -> CompositeMeasure<'a, F, M> {
    CompositeMeasure { function, measure }
}

/// A function which is the composition of two other functions.
pub struct CompositeFunction<
    'a,
    F: MeasurableFn<'a> + ?Sized,
    G: MeasurableFn<'a, Domain = F::Codomain> + ?Sized,
> {
    f: &'a F,
    g: &'a G,
}

impl<
        'subset,
        F: MeasurableFn<'subset> + ?Sized,
        G: MeasurableFn<'subset, Domain = F::Codomain> + ?Sized,
    > MeasurableFn<'subset> for CompositeFunction<'subset, F, G>
{
    type Domain = F::Domain;

    type Codomain = G::Codomain;

    #[with]
    fn preimage<'a>(
        &'a self,
        s: &'a <Self::Codomain as Measurable>::Subset<'a>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'ref>
    where
        'subset: 'a,
    {
        #[with]
        let s1 = self
            .g
            .preimage(<G::Codomain as Measurable>::subset_upcast(s));
        #[with]
        let s2 = self
            .f
            .preimage(<F::Codomain as Measurable>::subset_upcast(s1));
        s2
    }
}

/// Composes two functions.
pub fn compose<
    'a,
    F: MeasurableFn<'a> + ?Sized,
    G: MeasurableFn<'a, Domain = F::Codomain> + ?Sized,
>(
    g: &'a G,
    f: &'a F,
) -> CompositeFunction<'a, F, G> {
    CompositeFunction { f, g }
}

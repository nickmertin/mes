//! Implementation of binary Cartesian products as measurable spaces.

use derive_more::{Add, AddAssign, Mul, MulAssign};
use type_variance::{Contravariant, Invariant};
use with_locals::with;

use crate::{
    real::Real, util::proxy::Proxy, DiracMeasure, Measurable, MeasurableFn, Measure,
    PointMeasurable, PointMeasure,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A subset of [`bool`].
pub struct PairSubset<'a, T: Measurable + 'a, U: Measurable + ?Sized + 'a> {
    /// The subset of `T`.
    pub left: &'a T::Subset<'a>,

    /// The subset of `U`.
    pub right: &'a U::Subset<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The left projection function from `(T, U)` to `T`.
pub struct LeftProjection<T: Measurable, U: Measurable + ?Sized>(Invariant<T>, Contravariant<U>);

impl<'subset, T: Measurable + 'static, U: Measurable + ?Sized + 'static> MeasurableFn<'subset>
    for LeftProjection<T, U>
{
    type Domain = (T, U);

    type Codomain = T;

    #[with]
    fn preimage<'a>(
        &'a self,
        s: &'a <Self::Codomain as Measurable>::Subset<'a>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'ref>
    where
        'subset: 'a,
    {
        let right: &'ref _ = U::full_subset();
        &PairSubset {
            left: T::subset_upcast(s),
            right: U::subset_upcast(right),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The right projection function from `(T, U)` to `U`.
pub struct RightProjection<T: Measurable, U: Measurable + ?Sized>(Contravariant<T>, Invariant<U>);

impl<'subset, T: Measurable + 'static, U: Measurable + ?Sized + 'static> MeasurableFn<'subset>
    for RightProjection<T, U>
{
    type Domain = (T, U);

    type Codomain = U;

    #[with]
    fn preimage<'a>(
        &'a self,
        s: &'a <Self::Codomain as Measurable>::Subset<'a>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'ref>
    where
        'subset: 'a,
    {
        let left: &'ref _ = T::full_subset();
        &PairSubset {
            left: T::subset_upcast(left),
            right: U::subset_upcast(s),
        }
    }
}

/// The fork function, which, given functions from `T` to `U` and `T` to `V`,
/// gives a function from `T` to `(U, V)`.
pub struct ForkFunction<
    'a,
    T: Measurable,
    F: MeasurableFn<'a, Domain = T>,
    G: MeasurableFn<'a, Domain = T>,
> where
    F::Codomain: Sized + 'static,
    G::Codomain: 'static,
{
    left: &'a F,
    right: &'a G,
}

impl<
        'subset,
        T: Measurable,
        F: MeasurableFn<'subset, Domain = T>,
        G: MeasurableFn<'subset, Domain = T>,
    > MeasurableFn<'subset> for ForkFunction<'subset, T, F, G>
where
    F::Codomain: Sized + 'static,
    G::Codomain: 'static,
{
    type Domain = T;

    type Codomain = (F::Codomain, G::Codomain);

    #[with]
    fn preimage<'a>(
        &'a self,
        s: &'a <Self::Codomain as Measurable>::Subset<'a>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'ref>
    where
        'subset: 'a,
    {
        let left: &'ref _ = self.left.preimage(s.left);
        let right: &'ref _ = self.right.preimage(s.right);
        let result: &'ref _ =
            T::subset_intersect([T::subset_upcast(left), T::subset_upcast(right)].into_iter());
        result
    }
}

impl<T: Measurable + 'static, U: Measurable + ?Sized + 'static> Measurable for (T, U) {
    type Subset<'a> = PairSubset<'a, T, U> where Self: 'a;

    fn subset_upcast<'a, 'b: 'a>(s: &'a Self::Subset<'b>) -> &'a Self::Subset<'a> {
        // s
        todo!()
    }

    #[with]
    fn empty_subset() -> &'ref Self::Subset<'ref> {
        let left: &'ref _ = T::empty_subset();
        let right: &'ref _ = U::empty_subset();
        &PairSubset {
            left: T::subset_upcast(left),
            right: U::subset_upcast(right),
        }
    }

    #[with]
    fn full_subset() -> &'ref Self::Subset<'ref> {
        let left: &'ref _ = T::full_subset();
        let right: &'ref _ = U::full_subset();
        &PairSubset {
            left: T::subset_upcast(left),
            right: U::subset_upcast(right),
        }
    }

    fn subset_is_empty(s: &Self::Subset<'_>) -> bool {
        T::subset_is_empty(s.left) && U::subset_is_empty(s.right)
    }

    #[with]
    fn subset_complement(s: &Self::Subset<'_>) -> &'ref Self::Subset<'ref> {
        let PairSubset { left, right } = *s;
        let left_c: &'ref _ = T::subset_complement(left);
        let right_c: &'ref _ = U::subset_complement(right);
        let result: &'ref _ = Self::subset_union(
            [
                Proxy::new(&PairSubset {
                    left: T::subset_upcast(left),
                    right: U::subset_upcast(right_c),
                }),
                Proxy::new(&PairSubset {
                    left: T::subset_upcast(left_c),
                    right: U::subset_upcast(right),
                }),
                Proxy::new(&PairSubset {
                    left: T::subset_upcast(left_c),
                    right: U::subset_upcast(right_c),
                }),
            ]
            .iter(),
        );
        result
    }

    #[with]
    fn subset_union<'a>(
        subsets: impl Iterator<Item = &'a Proxy<'a, Self::Subset<'a>>> + Clone + 'a,
    ) -> &'ref Self::Subset<'ref>
    where
        Self: 'a,
    {
        // fn map_left<'a, 'b, T: Measurable + 'static, U: Measurable + ?Sized>(
        //     s: &'a PairSubset<'b, T, U>,
        //     f: &'a mut (dyn for<'c> FnMut(&'c T::Subset<'b>) + 'a),
        // ) {
        //     f(s.left);
        // }

        // let left: &'ref _ = T::subset_union(subsets.clone().map(|proxy|
        // proxy.map(&map_left))); let right: &'ref _ =
        //     U::subset_union(subsets.clone().map(|proxy| proxy.map(&|s, f|
        // f(s.right)))); &PairSubset {
        //     left: T::subset_upcast(left),
        //     right: U::subset_upcast(right),
        // }
        todo!()
    }
}

impl<T: PointMeasurable + 'static, U: PointMeasurable + ?Sized + 'static> PointMeasurable
    for (T, U)
{
    #[with]
    fn point_subset<'a>(&'a self) -> &'ref Self::Subset<'ref> {
        let left: &'ref _ = T::point_subset(&self.0);
        let right: &'ref _ = U::point_subset(&self.1);
        &PairSubset {
            left: T::subset_upcast(left),
            right: U::subset_upcast(right),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Add, AddAssign, Mul, MulAssign)]
/// A measure on [`bool`].
pub struct BoolMeasure<R: Real> {
    /// The value of the measure for `true`.
    pub true_value: R,

    /// The value of the measure for `false`.
    pub false_value: R,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
/// A probability measure on [`bool`].
pub struct BoolPMeasure<R: Real> {
    /// The value of the probability measure for `true`.
    true_value: R,
}

impl<R: Real> From<BoolPMeasure<R>> for BoolMeasure<R> {
    fn from(m: BoolPMeasure<R>) -> Self {
        Self {
            true_value: m.true_value,
            false_value: R::one() - m.true_value,
        }
    }
}

impl<'subset, R: Real> Measure<'subset> for BoolMeasure<R> {
    type R = R;

    type Space = bool;

    type Measurement = R;

    type PMeasure = BoolPMeasure<R>;

    #[with]
    fn measure<'a>(
        &'a self,
        domain: &'a <Self::Space as Measurable>::Subset<'a>,
    ) -> &'ref Self::Measurement
    where
        'subset: 'a,
    {
        let mut result = R::zero();
        if domain.includes_true {
            result += self.true_value;
        }
        if domain.includes_false {
            result += self.false_value;
        }
        &result
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        Some(BoolPMeasure {
            true_value: R::normalize_static([self.true_value, self.false_value])?[0],
        })
    }
}

impl<'subset, R: Real> PointMeasure<'subset> for BoolMeasure<R> {
    type PointMeasurement = R;

    #[with]
    fn measure_at(&self, value: &Self::Space) -> &'ref Self::PointMeasurement {
        if *value {
            &self.true_value
        } else {
            &self.false_value
        }
    }
}

impl<'subset, R: Real> DiracMeasure<'subset> for BoolMeasure<R> {
    fn dirac(point: &Self::Space) -> Self {
        if *point {
            Self {
                true_value: R::one(),
                false_value: R::zero(),
            }
        } else {
            Self {
                true_value: R::zero(),
                false_value: R::one(),
            }
        }
    }
}

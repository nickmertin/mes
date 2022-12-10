//! Implementation of [`bool`] as a measurable space.

use core::ops::Not;
use derive_more::{Add, AddAssign, Mul, MulAssign};
use with_locals::with;

use crate::{
    real::Real, DiracMeasure, Measurable, MeasurableFn, Measure, PointMeasurable, PointMeasure,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A subset of [`bool`].
pub struct BoolSubset {
    /// Whether the subset includes `true`.
    pub includes_true: bool,

    /// Whether the subset includes `false`.
    pub includes_false: bool,
}

/// A measurable function whose codomain is [`bool`].
pub struct BoolFunction<'a, T: Measurable + ?Sized + 'a> {
    /// The subset of the domain which maps to `true`.
    pub true_primage: T::Subset<'a>,
}

impl Not for BoolSubset {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            includes_true: !self.includes_true,
            includes_false: !self.includes_false,
        }
    }
}

impl<'subset, T: Measurable + ?Sized> MeasurableFn<'subset> for BoolFunction<'subset, T> {
    type Domain = T;

    type Codomain = bool;

    #[with]
    fn preimage<'a>(
        &'a self,
        s: &'a <Self::Codomain as Measurable>::Subset<'a>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'ref>
    where
        'subset: 'a,
    {
        match (s.includes_true, s.includes_false) {
            (true, true) => {
                let x: &'ref _ = T::full_subset();
                x
            }
            (true, false) => T::subset_upcast(&self.true_primage),
            (false, true) => {
                let x: &'ref _ = T::subset_complement(T::subset_upcast(&self.true_primage));
                x
            }
            (false, false) => {
                let x: &'ref _ = T::empty_subset();
                x
            }
        }
    }
}

impl Measurable for bool {
    type Subset<'a> = BoolSubset;

    fn subset_upcast<'a, 'b: 'a>(s: &'a Self::Subset<'b>) -> &'a Self::Subset<'a> {
        s
    }

    #[with]
    fn empty_subset() -> &'ref Self::Subset<'ref> {
        &BoolSubset {
            includes_true: false,
            includes_false: false,
        }
    }

    #[with]
    fn full_subset() -> &'ref Self::Subset<'ref> {
        &BoolSubset {
            includes_true: true,
            includes_false: true,
        }
    }

    fn subset_is_empty(s: &Self::Subset<'_>) -> bool {
        !s.includes_true && !s.includes_false
    }

    #[with]
    fn subset_complement(s: &Self::Subset<'_>) -> &'ref Self::Subset<'ref> {
        &!*s
    }

    #[with]
    fn subset_union<'a>(
        subsets: impl Iterator<Item = &'a Self::Subset<'a>> + Clone + 'a,
    ) -> &'ref Self::Subset<'ref>
    where
        Self: 'a,
    {
        let mut result = BoolSubset {
            includes_true: false,
            includes_false: false,
        };

        for s in subsets {
            result.includes_true |= s.includes_true;
            result.includes_false |= s.includes_false;
            if result.includes_true && result.includes_false {
                break;
            }
        }

        &result
    }
}

impl PointMeasurable for bool {
    #[with]
    fn point_subset(&self) -> &'ref Self::Subset<'ref> {
        &BoolSubset {
            includes_true: *self,
            includes_false: !*self,
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

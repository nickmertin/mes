//! Implementation of [`bool`] as a measurable space.

use core::ops::Not;
use derive_more::{Add, AddAssign, Mul, MulAssign};
use with_locals::with;

use crate::{
    real::Real, DiracMeasure, Measurable, MeasurableFn, Measure, PointMeasurable, PointMeasure,
    SigmaAlgebra,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolSubset {
    pub includes_true: bool,
    pub includes_false: bool,
}

pub struct BoolFunction<'a, T: Measurable + ?Sized> {
    pub true_partition: T::Subset<'a>,
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

impl SigmaAlgebra<'_> for BoolSubset {
    type Space = bool;

    #[with]
    fn empty() -> &'ref Self {
        &Self {
            includes_true: false,
            includes_false: false,
        }
    }

    #[with]
    fn full() -> &'ref Self {
        &Self {
            includes_true: true,
            includes_false: true,
        }
    }

    fn is_empty(&self) -> bool {
        !self.includes_true && !self.includes_false
    }

    #[with]
    fn inversion(&self) -> &'ref Self {
        &!*self
    }
}

impl<'subset, T: Measurable + ?Sized> MeasurableFn<'subset> for BoolFunction<'subset, T> {
    type Domain = T;

    type Codomain = bool;

    #[with]
    fn preimage<'a>(
        f: &'a Self,
        s: &'a <Self::Codomain as Measurable>::Subset<'a>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'a>
    where
        'subset: 'a,
    {
        match (s.includes_true, s.includes_false) {
            (true, true) => {
                #[with]
                let x = T::Subset::full();
                &x
            }
            (true, false) => T::subset_upcast(&f.true_partition),
            (false, true) => {
                #[with]
                let x = T::subset_upcast(&f.true_partition).inversion();
                &x
            }
            (false, false) => {
                #[with]
                let x = T::Subset::empty();
                &x
            }
        }
    }
}

impl Measurable for bool {
    type Subset<'a> = BoolSubset;

    fn subset_upcast<'a, 'b: 'a>(s: &'a Self::Subset<'b>) -> &'a Self::Subset<'a> {
        s
    }
}

impl PointMeasurable for bool {
    #[with]
    fn point_subset(&self) -> &'ref Self::Subset<'_> {
        &BoolSubset {
            includes_true: *self,
            includes_false: !*self,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Add, AddAssign, Mul, MulAssign)]
struct BoolMeasure<R: Real> {
    true_value: R,
    false_value: R,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct BoolPMeasure<R: Real>(R);

impl<R: Real> From<BoolPMeasure<R>> for BoolMeasure<R> {
    fn from(m: BoolPMeasure<R>) -> Self {
        Self {
            true_value: m.0,
            false_value: R::one() - m.0,
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
        Some(BoolPMeasure(
            R::normalize_static([self.true_value, self.false_value])?[0],
        ))
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
    fn point(value: &Self::Space) -> Self {
        if *value {
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

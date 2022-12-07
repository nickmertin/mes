//! Implementation of (univariate) Dirac delta measure.

use core::ops::{Mul, MulAssign};
use num_traits::float::FloatCore;
use with_locals::with;

use crate::{DiracMeasure, Measurable, Measure, PointMeasure};

use super::Real;

#[derive(PartialEq)]
///A (univariate) weighted Dirac delta measure.
pub struct Dirac<R: Real> {
    /// The location of the point mass.
    pub point: R,

    /// THe weight of the measure.
    pub weight: R,
}

#[derive(PartialEq, PartialOrd)]
///A (univariate) Dirac delta probability measure.
pub struct PDirac<R: Real> {
    /// The location of the point mass.
    pub point: R,
}

impl<R: Real> From<PDirac<R>> for Dirac<R> {
    fn from(m: PDirac<R>) -> Self {
        Self {
            point: m.point,
            weight: R::one(),
        }
    }
}

impl<R: Real> Mul<R> for Dirac<R> {
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        Self {
            point: self.point,
            weight: self.weight * rhs,
        }
    }
}

impl<R: Real> MulAssign<R> for Dirac<R> {
    fn mul_assign(&mut self, rhs: R) {
        self.weight *= rhs;
    }
}

impl<'subset, R: Real + FloatCore> Measure<'subset> for Dirac<R> {
    type R = R;

    type Space = R;

    type Measurement = R;

    type PMeasure = PDirac<R>;

    #[with]
    fn measure<'a>(
        &'a self,
        domain: &'a <Self::Space as Measurable>::Subset<'a>,
    ) -> &'ref Self::Measurement
    where
        'subset: 'a,
    {
        if domain.contains(&self.point) {
            &self.weight
        } else {
            &R::zero()
        }
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        R::normalize_static([self.weight])?;
        Some(PDirac { point: self.point })
    }
}

impl<'subset, R: Real + FloatCore> PointMeasure<'subset> for Dirac<R> {
    type PointMeasurement = R;

    #[with]
    fn measure_at(&self, value: &Self::Space) -> &'ref Self::PointMeasurement {
        &if *value == self.point {
            R::infinity()
        } else {
            R::zero()
        }
    }
}

impl<'subset, R: Real + FloatCore> DiracMeasure<'subset> for Dirac<R> {
    fn point(value: &Self::Space) -> Self {
        Self {
            point: *value,
            weight: R::one(),
        }
    }
}

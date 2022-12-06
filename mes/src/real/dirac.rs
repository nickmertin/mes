//! Implementation of (univariate) Dirac delta distributions.

use core::ops::{Mul, MulAssign};
use num_traits::float::FloatCore;

use crate::measure::Measure;

use super::Real;

#[derive(PartialEq)]
///A (univariate) weighted Dirac delta measure.
pub struct Dirac<R: Real> {
    pub point: R,
    pub weight: R,
}

#[derive(PartialEq, PartialOrd)]
///A (univariate) Dirac delta distribution.
pub struct PDirac<R: Real> {
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
        self.weight *= rhs
    }
}

impl<R: Real + FloatCore> Measure for Dirac<R> {
    type R = R;

    type Space = R;

    type Measurement<'a> = R
    where
        Self: 'a;

    type PMeasure = PDirac<R>;

    fn measure_at(&self, value: &Self::Space) -> Self::Measurement<'_> {
        if *value == self.point {
            R::infinity()
        } else {
            R::zero()
        }
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        R::normalize_static([self.weight])?;
        Some(PDirac { point: self.point })
    }
}

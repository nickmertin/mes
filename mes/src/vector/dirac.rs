//! Implementation of multivariate Dirac delta distributions.

use core::ops::{Mul, MulAssign};
use num_traits::float::FloatCore;

use crate::measure::Measure;

use super::{Real2, Vector};

#[derive(PartialEq)]
pub struct VDirac<R: Real2, const D: usize> {
    pub point: Vector<R, D>,
    pub weight: R,
}

#[derive(PartialEq)]
pub struct PVDirac<R: Real2, const D: usize> {
    pub point: Vector<R, D>,
}

impl<R: Real2, const D: usize> From<PVDirac<R, D>> for VDirac<R, D> {
    fn from(m: PVDirac<R, D>) -> Self {
        Self {
            point: m.point,
            weight: R::one(),
        }
    }
}

impl<R: Real2, const D: usize> Mul<R> for VDirac<R, D> {
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        Self {
            point: self.point,
            weight: self.weight * rhs,
        }
    }
}

impl<R: Real2, const D: usize> MulAssign<R> for VDirac<R, D> {
    fn mul_assign(&mut self, rhs: R) {
        self.weight *= rhs
    }
}

impl<R: Real2 + FloatCore, const D: usize> Measure for VDirac<R, D> {
    type R = R;

    type Space = Vector<R, D>;

    type Measurement<'a> = R
    where
        Self: 'a;

    type PMeasure = PVDirac<R, D>;

    fn measure_at(&self, value: &Self::Space) -> Self::Measurement<'_> {
        if *value == self.point {
            R::infinity()
        } else {
            R::zero()
        }
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        R::normalize_static([self.weight])?;
        Some(PVDirac { point: self.point })
    }
}

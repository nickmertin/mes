//! Implementation of (univariate) Gaussian distributions.

#![cfg(any(feature = "libm", feature = "std"))]
#![cfg_attr(doc_cfg, doc(cfg(any(feature = "libm", feature = "std"))))]

use core::ops::{Mul, MulAssign};
use derive_more::{Add, AddAssign};

use crate::measure::Measure;

use super::{dirac::Dirac, Real};

#[derive(Debug, Clone, Copy, PartialEq)]
/// A (univariate) weighted Gaussian measure.
pub struct Gaussian<R: Real> {
    pub distribution: PGaussian<R>,
    pub weight: R,
}

/// A (univariate) Gaussian distribution.
#[derive(Debug, Clone, Copy, PartialEq, Add, AddAssign)]
pub struct PGaussian<R: Real> {
    pub mean: R,
    pub variance: R,
}

impl<R: Real> From<Dirac<R>> for Gaussian<R> {
    fn from(m: Dirac<R>) -> Self {
        Self {
            distribution: PGaussian {
                mean: m.point,
                variance: R::zero(),
            },
            weight: m.weight,
        }
    }
}

impl<R: Real> From<PGaussian<R>> for Gaussian<R> {
    fn from(distribution: PGaussian<R>) -> Self {
        Self {
            distribution,
            weight: R::one(),
        }
    }
}

impl<R: Real> Mul<R> for Gaussian<R> {
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        Self {
            distribution: self.distribution,
            weight: self.weight * rhs,
        }
    }
}

impl<R: Real> MulAssign<R> for Gaussian<R> {
    fn mul_assign(&mut self, rhs: R) {
        self.weight *= rhs;
    }
}

impl<R: Real> Measure for Gaussian<R> {
    type R = R;

    type Space = R;

    type Measurement<'a> = R
    where
        Self: 'a;

    type PMeasure = PGaussian<R>;

    fn measure_at(&self, value: &Self::Space) -> Self::Measurement<'_> {
        let half = (R::one() + R::one()).recip();

        let PGaussian { mean, variance } = self.distribution;
        let offset = *value - mean;
        self.weight
            * (variance * R::two_pi()).sqrt().recip()
            * (-half / variance * offset * offset).exp()
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        R::normalize_static([self.weight])?;
        Some(self.distribution)
    }
}

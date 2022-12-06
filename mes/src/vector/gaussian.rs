//! Implementaiton of the multivariate Gaussian distribution.

#![cfg(any(feature = "libm", feature = "std"))]
#![cfg_attr(doc_cfg, doc(cfg(any(feature = "libm", feature = "std"))))]

use core::ops::{Mul, MulAssign};
use derive_more::{Add, AddAssign};
use nalgebra::{Const, DimMin};
use num_traits::Zero;

use crate::{
    measure::Measure,
    real::{
        gaussian::{Gaussian, PGaussian},
        Real,
    },
};

use super::{dirac::VDirac, Matrix, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
/// A (univariate) weighted Gaussian measure.
pub struct VGaussian<R: Real, const D: usize> {
    pub distribution: PVGaussian<R, D>,
    pub weight: R,
}

/// A (univariate) Gaussian distribution.
#[derive(Debug, Clone, Copy, PartialEq, Add, AddAssign)]
pub struct PVGaussian<R: Real, const D: usize> {
    pub location: Vector<R, D>,
    pub covariance: Matrix<R, D, D>, // TODO: make this more efficient
}

impl<R: Real> From<Gaussian<R>> for VGaussian<R, 1> {
    fn from(m: Gaussian<R>) -> Self {
        Self {
            distribution: m.distribution.into(),
            weight: m.weight,
        }
    }
}

impl<R: Real> From<PGaussian<R>> for PVGaussian<R, 1> {
    fn from(p: PGaussian<R>) -> Self {
        Self {
            location: Matrix([p.mean].into()),
            covariance: Matrix([p.variance].into()),
        }
    }
}

impl<R: Real, const D: usize> From<VDirac<R, D>> for VGaussian<R, D> {
    fn from(m: VDirac<R, D>) -> Self {
        Self {
            distribution: PVGaussian {
                location: m.point,
                covariance: Matrix::zero(),
            },
            weight: m.weight,
        }
    }
}

impl<R: Real, const D: usize> From<PVGaussian<R, D>> for VGaussian<R, D> {
    fn from(distribution: PVGaussian<R, D>) -> Self {
        Self {
            distribution,
            weight: R::one(),
        }
    }
}

impl<R: Real, const D: usize> Mul<R> for VGaussian<R, D> {
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        Self {
            distribution: self.distribution,
            weight: self.weight * rhs,
        }
    }
}

impl<R: Real, const D: usize> MulAssign<R> for VGaussian<R, D> {
    fn mul_assign(&mut self, rhs: R) {
        self.weight *= rhs;
    }
}

impl<R: Real, const D: usize> Measure for VGaussian<R, D>
where
    Const<D>: DimMin<Const<D>, Output = Const<D>>,
{
    type R = R;

    type Space = Vector<R, D>;

    type Measurement<'a> = R
    where
        Self: 'a;

    type PMeasure = PVGaussian<R, D>;

    fn measure_at(&self, value: &Self::Space) -> Self::Measurement<'_> {
        let half = (R::one() + R::one()).recip();

        let PVGaussian {
            location: mean,
            covariance: variance,
        } = self.distribution;
        let offset = *value - mean;

        self.weight
            * (R::two_pi().powi(D as i32) * variance.0.determinant().recip()).sqrt()
            * (-half
                * Into::<[R; 1]>::into(
                    offset.0.transpose() * variance.0.try_inverse().unwrap() * offset.0,
                )[0])
                .exp()
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        R::normalize_static([self.weight])?;
        Some(self.distribution)
    }
}

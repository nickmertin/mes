//! Implementaiton of the multivariate Gaussian distribution.

#![cfg(any(feature = "libm", feature = "std"))]
#![cfg_attr(doc_cfg, doc(cfg(any(feature = "libm", feature = "std"))))]

use core::ops::{Mul, MulAssign};
use derive_more::{Add, AddAssign};
use nalgebra::{Const, DimMin};
use num_traits::{float::Float, FloatConst, Zero};

use crate::measure::Measure;

use super::{dirac::VDirac, Matrix, Real2, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
/// A (univariate) weighted Gaussian measure.
pub struct VGaussian<R: Real2, const D: usize> {
    distribution: PVGaussian<R, D>,
    weight: R,
}

/// A (univariate) Gaussian distribution.
#[derive(Debug, Clone, Copy, PartialEq, Add, AddAssign)]
pub struct PVGaussian<R: Real2, const D: usize> {
    mean: Vector<R, D>,
    variance: Matrix<R, D, D>, // TODO: make this more efficient
}

impl<R: Real2, const D: usize> From<VDirac<R, D>> for VGaussian<R, D> {
    fn from(m: VDirac<R, D>) -> Self {
        Self {
            distribution: PVGaussian {
                mean: m.point,
                variance: Matrix::zero(),
            },
            weight: m.weight,
        }
    }
}

impl<R: Real2, const D: usize> From<PVGaussian<R, D>> for VGaussian<R, D> {
    fn from(distribution: PVGaussian<R, D>) -> Self {
        Self {
            distribution,
            weight: R::one(),
        }
    }
}

impl<R: Real2, const D: usize> Mul<R> for VGaussian<R, D> {
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        Self {
            distribution: self.distribution,
            weight: self.weight * rhs,
        }
    }
}

impl<R: Real2, const D: usize> MulAssign<R> for VGaussian<R, D> {
    fn mul_assign(&mut self, rhs: R) {
        self.weight *= rhs;
    }
}

impl<R: Real2 + Float + FloatConst, const D: usize> Measure for VGaussian<R, D>
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
        let half = Float::recip((R::one() + R::one()));

        let PVGaussian { mean, variance } = self.distribution;
        let offset = *value - mean;
        // (variance * Vector<R, D>::TAU()).sqrt().recip() * (-half / variance * offset * offset).exp()

        // TODO: switch to using simba::RealField
        todo!()

        // Float::powi(Float::sqrt(R::TAU()), -(D as i32)) * variance.0.determinant()
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        R::normalize_static([self.weight])?;
        Some(self.distribution)
    }
}

// #[repr(transparent)]
// #[derive(Debug, PartialEq, PartialOrd, Add, Sub)]
// struct Wrapper<R: Real2>(R);

// impl<R: Real2> ComplexField for Wrapper<R> {}

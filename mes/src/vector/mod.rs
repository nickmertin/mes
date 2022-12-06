//! Facilities for working with vectors of real numbers.

#![cfg(feature = "vector")]
#![cfg_attr(doc_cfg, doc(cfg(feature = "vector")))]

use core::ops::{Mul, MulAssign};

use derive_more::{Add, AddAssign, Sub, SubAssign};
use nalgebra::SMatrix;
use num_traits::{One, Zero};

use crate::real::Real;

pub mod dirac;
pub mod gaussian;

pub type Vector<R, const D: usize> = Matrix<R, D, 1>;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Add, AddAssign, Sub, SubAssign)]
pub struct Matrix<R: Real, const A: usize, const B: usize>(SMatrix<R, A, B>);

impl<R: Real, const A: usize, const B: usize, const C: usize> Mul<Matrix<R, B, C>>
    for Matrix<R, A, B>
{
    type Output = Matrix<R, A, C>;

    fn mul(self, rhs: Matrix<R, B, C>) -> Self::Output {
        Matrix(self.0 * rhs.0)
    }
}

impl<R: Real, const D: usize> MulAssign for Matrix<R, D, D> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl<R: Real, const A: usize, const B: usize> Zero for Matrix<R, A, B> {
    fn zero() -> Self {
        Self(SMatrix::zeros())
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl<R: Real, const D: usize> One for Matrix<R, D, D> {
    fn one() -> Self {
        Self(SMatrix::identity())
    }
}

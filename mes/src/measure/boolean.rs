use derive_more::{Add, AddAssign, Mul, MulAssign};

use crate::real::Real;

use super::{DiracMeasure, Measure};

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

// impl<R: Real> Mul<R> for BoolMeasure<R> {
//     type Output = Self;

//     fn mul(self, rhs: R) -> Self::Output {
//         Self {
//             true_value: self.true_value * rhs,
//             false_value: self.false_value * rhs,
//         }
//     }
// }

// impl<R: Real> MulAssign<R> for BoolMeasure<R> {
//     fn mul_assign(&mut self, rhs: R) {
//         *self = *self * rhs
//     }
// }

impl<R: Real> Measure for BoolMeasure<R> {
    type R = R;

    type Space = bool;

    type Measurement<'a> = R
    where
        Self: 'a;

    type PMeasure = BoolPMeasure<R>;

    fn measure_at(&self, value: &Self::Space) -> Self::Measurement<'_> {
        if *value {
            self.true_value
        } else {
            self.false_value
        }
    }

    fn normalize(&self) -> Option<Self::PMeasure> {
        Some(BoolPMeasure(
            R::normalize_static([self.true_value, self.false_value])?[0],
        ))
    }
}

impl<R: Real> DiracMeasure for BoolMeasure<R> {
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

// impl<R: Real> Measure for BoolPMeasure<R> {
//     type R = R;

//     type Space = bool;

//     type Measurement<'a> = R
//     where
//         Self: 'a;

//     type PMeasure = Self;

//     fn measure_at(&self, value: &Self::Space) -> Self::Measurement<'_> {
//         if *value {
//             self.0
//         } else {
//             R::one() - self.0
//         }
//     }

//     fn normalize(&self) -> Option<Self::PMeasure> {
//         Some(*self)
//     }
// }

//! Facilities for working with real numbers.

use simba::scalar::RealField;
use with_locals::with;

use crate::{measurable::Measurable, sigma::SigmaAlgebra};

pub mod dirac;
// pub mod gaussian;

/// Describes a type which represents a real number.
pub trait Real: RealField + Copy + 'static {
    /// Normalizes the list of numbers.
    fn normalize(nums: &mut [Self]) -> Option<()>;

    #[inline]
    /// Normalizes the list of numbers.
    fn normalize_static<const N: usize>(mut nums: [Self; N]) -> Option<[Self; N]> {
        Self::normalize(&mut nums)?;
        Some(nums)
    }
}

macro_rules! impl_real {
    ($type:ty) => {
        impl Real for $type {
            fn normalize(nums: &mut [Self]) -> Option<()> {
                let sum: Self = nums.iter().map(|x| *x).sum();
                let factor = sum.recip();
                if !factor.is_finite() {
                    return None;
                }

                nums.iter_mut().for_each(|x| *x *= factor);
                Some(())
            }
        }
    };
}

impl_real!(f32);
impl_real!(f64);

pub trait RealSubset<R: Real> {
    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;

    fn contains(&self, value: &R) -> bool;
}

// pub trait RealFunction<T: Measurable + ?Sized, R: Real> {}

// impl<'a, R: Real> From<&'a RealSubsetCore<[(bool, R)]>> for &'a RealSubset<R>
// {     fn from(core: &'a RealSubsetCore<[(bool, R)]>) -> Self {
//         unsafe { &*(core as *const _ as *const Self) }
//     }
// }

impl<'a, R: Real> SigmaAlgebra<'a> for dyn RealSubset<R> + 'a {
    type Space = R;

    #[with]
    fn empty() -> &'ref Self {
        struct EmptySubset;

        impl<R: Real> RealSubset<R> for EmptySubset {
            fn is_empty(&self) -> bool {
                true
            }

            fn is_full(&self) -> bool {
                false
            }

            fn contains(&self, _value: &R) -> bool {
                false
            }
        }

        &EmptySubset
    }

    #[with]
    fn full() -> &'ref Self {
        struct FullSubset(u8);

        impl<R: Real> RealSubset<R> for FullSubset {
            fn is_empty(&self) -> bool {
                false
            }

            fn is_full(&self) -> bool {
                true
            }

            fn contains(&self, _value: &R) -> bool {
                true
            }
        }

        &FullSubset(0)
    }

    fn is_empty(&'a self) -> bool {
        // !self.0.left_unbounded && self.0.points.is_empty()
        self.is_empty()
    }

    #[with]
    fn inversion(&'a self) -> &'ref Self {
        struct InverseSubset<'x, R>(&'x (dyn RealSubset<R> + 'x));

        impl<'x, R: Real> RealSubset<R> for InverseSubset<'x, R> {
            fn is_empty(&self) -> bool {
                self.0.is_full()
            }

            fn is_full(&self) -> bool {
                self.0.is_empty()
            }

            fn contains(&self, value: &R) -> bool {
                !self.0.contains(value)
            }
        }

        &InverseSubset(self)
    }
}

impl<R: Real> Measurable for R {
    type Subset<'a> = dyn RealSubset<R> + 'a;

    fn subset_upcast<'a, 'b: 'a>(s: &'a Self::Subset<'b>) -> &'a Self::Subset<'a> {
        s
    }

    // type Function<'a, T: Measurable + ?Sized + 'a> = dyn RealFunction<T, R> + 'a;

    // fn with_preimage<'a, T: Measurable + ?Sized + 'a, U>(
    //     f: &Self::Function<'a, T>,
    //     s: &Self::Subset<'a>,
    //     g: impl FnOnce(&'a T::Subset<'a>) -> U,
    // ) -> U {
    //     todo!()
    // }
}

// impl<F: RealField + Copy + Sum<F>> Real for F {
//     fn normalize(nums: &mut [Self]) -> Option<()> {
//         let sum: Self = nums.iter().map(|x| *x).sum();
//         let factor = sum.recip();
//         if !factor.is_finite() {
//             return None;
//         }

//         nums.iter_mut().for_each(|x| *x *= factor);
//         Some(())
//     }
// }

/// Describes a probability distribution over real numbers.
pub trait RealDistribution {
    /// The type of real number used for measure values in the distribution.
    type R: Real;
}

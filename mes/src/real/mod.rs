//! Facilities for working with real numbers.

use num_traits::Num;

/// Describes a type which represents a real number.
pub trait Real: Copy + PartialOrd + Num {
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
                let sum: Self = nums.iter().sum();
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

// impl<R: Copy + PartialOrd + NumAssign + Float + for<'a> Sum<&'a Self>> Real
// for R {     fn normalize(nums: &mut [Self]) -> Option<()> {
//         let sum: Self = nums.iter().sum();
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

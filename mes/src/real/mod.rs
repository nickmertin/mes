//! Facilities for working with real numbers.

use core::iter::Sum;
use simba::scalar::RealField;

pub mod dirac;
pub mod gaussian;

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

impl<F: RealField + Copy + Sum<F>> Real for F {
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

/// Describes a probability distribution over real numbers.
pub trait RealDistribution {
    /// The type of real number used for measure values in the distribution.
    type R: Real;
}

//! Facilities for working with real numbers.

use num_traits::{float::FloatCore, NumAssign};
use with_locals::with;

use crate::Measurable;

pub mod dirac;
// pub mod gaussian;

/// Describes a type which represents a real number.
pub trait Real: FloatCore + NumAssign + Copy + 'static {
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

/// Describes a subset of the real number line.
pub trait RealSubset<R: Real> {
    /// Checks whether the subset is empty.
    fn is_empty(&self) -> bool;

    /// Checks whether the subset is full (i.e., contains the entire real line).
    fn is_full(&self) -> bool;

    /// Checks whether the subset contains the given value.
    fn contains(&self, value: &R) -> bool;
}

impl<R: Real> Measurable for R {
    type Subset<'a> = dyn RealSubset<R> + 'a;

    fn subset_upcast<'a, 'b: 'a>(s: &'a Self::Subset<'b>) -> &'a Self::Subset<'a> {
        s
    }

    #[with]
    fn empty_subset() -> &'ref Self::Subset<'ref> {
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
    fn full_subset() -> &'ref Self::Subset<'ref> {
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

    fn subset_is_empty(s: &Self::Subset<'_>) -> bool {
        // !self.0.left_unbounded && self.0.points.is_empty()
        s.is_empty()
    }

    #[with]
    fn subset_complement(s: &Self::Subset<'_>) -> &'ref Self::Subset<'ref> {
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

        &InverseSubset(s)
    }

    #[with]
    fn subset_union<'a>(
        subsets: impl Iterator<Item = &'a Self::Subset<'a>> + Clone,
    ) -> &'ref Self::Subset<'ref>
    where
        Self: 'a,
    {
        struct UnionSubset<T>(T);

        impl<'x, R: Real, T: Iterator<Item = &'x <R as Measurable>::Subset<'x>> + Clone>
            RealSubset<R> for UnionSubset<T>
        {
            fn is_empty(&self) -> bool {
                self.0.clone().all(|s| s.is_empty())
            }

            fn is_full(&self) -> bool {
                todo!()
            }

            fn contains(&self, value: &R) -> bool {
                self.0.clone().all(|s| s.is_empty())
            }
        }

        &UnionSubset(subsets)
    }
}

// impl<R: Real> PointMeasurable for R {
//     #[with]
//     fn point_subset(&self) -> &'ref Self::Subset<'_> {
//         &BoolSubset {
//             includes_true: *self,
//             includes_false: !*self,
//         }
//     }
// }

/// Describes a probability distribution over real numbers.
pub trait RealDistribution {
    /// The type of real number used for measure values in the distribution.
    type R: Real;
}

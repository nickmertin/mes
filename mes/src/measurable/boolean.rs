//! Implementation of the unit type as a measurable space.

use core::ops::Not;

use with_locals::with;

use crate::{measure::Measure, sigma::SigmaAlgebra};

use super::{Measurable, MeasurableFn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolSubset {
    pub includes_true: bool,
    pub includes_false: bool,
}

pub struct BoolFunction<'a, T: Measurable + ?Sized> {
    pub true_partition: T::Subset<'a>,
}

impl Not for BoolSubset {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            includes_true: !self.includes_true,
            includes_false: !self.includes_false,
        }
    }
}

impl SigmaAlgebra<'_> for BoolSubset {
    type Space = bool;

    #[with]
    fn empty() -> &'ref Self {
        &Self {
            includes_true: false,
            includes_false: false,
        }
    }

    #[with]
    fn full() -> &'ref Self {
        &Self {
            includes_true: true,
            includes_false: true,
        }
    }

    fn is_empty(&self) -> bool {
        !self.includes_true && !self.includes_false
    }

    #[with]
    fn inversion(&self) -> &'ref Self {
        &!*self
    }
}

impl<'a, T: Measurable + ?Sized> MeasurableFn for BoolFunction<'a, T> {
    type Domain = T;

    type Codomain = bool;

    #[with]
    fn preimage<'subset>(
        f: &Self,
        s: &<Self::Codomain as Measurable>::Subset<'subset>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'subset> {
        // match (s.includes_true, s.includes_false) {
        //     (true, true) => T::Subset::with_full(g),
        //     (true, false) => g(&f.true_partition),
        //     (false, true) => f.true_partition.with_inversion(g),
        //     (false, false) => T::Subset::with_empty(g),
        // }
        todo!()
    }
}

impl Measurable for bool {
    type Subset<'a> = BoolSubset;

    // type Function<'a, T: Measurable + ?Sized + 'a> = BoolFunction<'a, T>;

    // fn with_preimage<'a, T: Measurable + ?Sized + 'a, U>(
    //     f: &'a Self::Function<'a, T>,
    //     s: &'a Self::Subset<'a>,
    //     g: impl FnOnce(&'a T::Subset<'a>) -> U,
    // ) -> U {
    //     match (s.includes_true, s.includes_false) {
    //         (true, true) => T::Subset::with_full(g),
    //         (true, false) => g(&f.true_partition),
    //         (false, true) => f.true_partition.with_inversion(g),
    //         (false, false) => T::Subset::with_empty(g),
    //     }
    // }

    // fn integrate<'a, T: Measurable + ?Sized, M: Measure<Space = Self> + ?Sized>(
    //     domain: &'a T::Subset,
    //     f: &'a Self::Function<T>,
    //     m: &'a M,
    // ) -> M::Measurement<'a> {
    //     // m.measure(&BoolSubset(!domain.is_empty()))
    //     todo!()
    // }
}

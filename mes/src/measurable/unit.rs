//! Implementation of the unit type as a measurable space.

use type_variance::Contravariant;
use with_locals::with;

use crate::{measure::Measure, sigma::SigmaAlgebra};

use super::{Measurable, MeasurableFn};

pub struct UnitSubset {
    pub full: bool,
}

pub struct UnitFunction<T: ?Sized>(Contravariant<T>);

impl SigmaAlgebra<'_> for UnitSubset {
    type Space = ();

    #[with]
    fn empty() -> &'ref Self {
        &Self { full: false }
    }

    #[with]
    fn full() -> &'ref Self {
        &Self { full: true }
    }

    fn is_empty(&self) -> bool {
        !self.full
    }

    #[with]
    fn inversion(&self) -> &'ref Self {
        &Self { full: !self.full }
    }
}

impl<T: Measurable + ?Sized> MeasurableFn for UnitFunction<T> {
    type Domain = T;

    type Codomain = ();

    #[with]
    fn preimage<'subset>(
        _f: &Self,
        s: &<Self::Codomain as Measurable>::Subset<'subset>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'subset> {
        if s.full {
            #[with]
            let x = T::Subset::full();
            x
        } else {
            #[with]
            let x = T::Subset::empty();
            x
        }
    }
}

impl Measurable for () {
    type Subset<'a> = UnitSubset;

    // type Function<'a, T: Measurable + ?Sized + 'a> = UnitFunction;

    // fn with_preimage<'a, T: Measurable + ?Sized + 'a, U>(
    //     _f: &'a Self::Function<'a, T>,
    //     s: &'a Self::Subset<'a>,
    //     g: impl FnOnce(&'a T::Subset<'a>) -> U,
    // ) -> U {
    //     if s.0 {
    //         T::Subset::with_full(g)
    //     } else {
    //         T::Subset::with_empty(g)
    //     }
    // }

    // fn integrate<'a, T: Measurable + ?Sized, M: Measure<Space = Self> + ?Sized>(
    //     domain: &'a T::Subset,
    //     f: &'a Self::Function<T>,
    //     m: &'a M,
    // ) -> M::Measurement<'a> {
    //     m.measure(&UnitSubset(!domain.is_empty()))
    // }
}

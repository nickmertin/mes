//! Implementation of the unit type as a measurable space.

use type_variance::Contravariant;

use crate::{measure::Measure, sigma::SigmaAlgebra};

use super::{Measurable, MeasurableFn};

pub struct UnitSubset {
    pub full: bool,
}

pub struct UnitFunction<T: ?Sized>(Contravariant<T>);

impl<'a> SigmaAlgebra<'a> for UnitSubset {
    type Space = ();

    fn with_empty<U>(f: impl FnOnce(&'a Self) -> U) -> U {
        f(&Self { full: false })
    }

    fn with_full<U>(f: impl FnOnce(&'a Self) -> U) -> U {
        f(&Self { full: true })
    }

    fn is_empty(&self) -> bool {
        !self.full
    }

    fn with_inversion<U>(&'a self, f: impl FnOnce(&Self) -> U) -> U {
        f(&Self { full: !self.full })
    }
}

impl<'a, T: Measurable + ?Sized + 'a> MeasurableFn<'a> for UnitFunction<T> {
    type Domain = T;

    type Codomain = ();

    fn with_preimage<'b: 'a, 'c, U>(
        _f: &'a Self,
        s: &'c <Self::Codomain as Measurable>::Subset<'b>,
        g: impl FnOnce(&<Self::Domain as Measurable>::Subset<'c>) -> U + 'c,
    ) -> U {
        if s.full {
            T::Subset::with_full(g)
        } else {
            T::Subset::with_empty(g)
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

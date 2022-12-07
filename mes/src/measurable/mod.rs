use with_locals::with;

mod measure;
mod sigma;

pub use measure::*;
pub use sigma::*;

/// Describes a data type as a measurable space.
pub trait Measurable {
    /// The type representing measurable subsets of [`Self`].
    type Subset<'a>: SigmaAlgebra<'a, Space = Self> + ?Sized;

    /// Upcasts a [`Self::Subset`] reference by lifetime. Essentially a proof
    /// that [`Self::Subset`] is covariant with respect to `'a`.
    ///
    /// Inspired by: <https://internals.rust-lang.org/t/variance-of-lifetime-arguments-in-gats/14769/19>
    ///
    /// The implementation should just be:
    /// ```rust
    /// fn subset_upcast<'a, 'b: 'a>(s: &'a Self::Subset<'b>) -> &'a Self::Subset<'a> {
    ///     s
    /// }
    /// ```
    ///
    /// If that does not compile, then your definition of [`Self::Subset`] is
    /// probably not covariant.
    fn subset_upcast<'a, 'b: 'a>(s: &'a Self::Subset<'b>) -> &'a Self::Subset<'a>;

    // /// The type representing measurable functions from `T` to [`Self`].
    // type Function<'a, T: Measurable + ?Sized + 'a>: ?Sized + 'a;

    // fn with_preimage<'a, T: Measurable + ?Sized + 'a, U>(
    //     f: &'a Self::Function<'a, T>,
    //     s: &'a Self::Subset<'a>,
    //     g: impl FnOnce(&'a T::Subset<'_>) -> U,
    // ) -> U;

    // fn with_integral<'a, T: Measurable + ?Sized, M: Measure<Space = Self> +
    // ?Sized, U>(     domain: &'a T::Subset,
    //     f: &'a Self::Function<T>,
    //     m: &'a M,
    // ) -> U;

    // /// Describes an arbitrary finite measure over [`Self`].
    // type Measure<R: Real>: ?Sized;

    // /// Describes an arbitrary probability measure over [`Self`].
    // type PMeasure<R: Real>: ?Sized;

    // /// Produces the zero measure.
    // fn zero<R: Real>() -> Self::Measure<R>
    // where
    //     Self::Measure<R>: Sized;

    // /// Computes the total measure.
    // fn total<R: Real>(m: &Self::Measure<R>) -> R;

    // /// Normalizes a finite measure into a probability measure. Returns [`None`]
    // /// if the measure cannot be normalized (e.g., its total measure is zero).
    // /// This can only be used when [`Self::PMeasure`] implements [`Sized`].
    // fn normalize<R: Real>(m: &Self::Measure<R>) -> Option<Self::PMeasure<R>>
    // where
    //     Self::PMeasure<R>: Sized;

    // /// Normalizes a finite measure into a probability measure, and passes it to
    // /// the given function `f`. Returns [`None`] and does not call `f` if
    // /// the measure cannot be normalized (e.g., its total measure is zero).
    // fn with_normalized<R: Real, T>(
    //     m: &Self::Measure<R>,
    //     f: impl for<'a> FnOnce(&'a Self::PMeasure<R>) -> T,
    // ) -> Option<T>;
}

pub trait PointMeasurable: Measurable {
    #[with]
    fn point_subset(&self) -> &'ref Self::Subset<'_>;
}

pub trait MeasurableFn<'subset> {
    type Domain: Measurable + ?Sized;

    type Codomain: Measurable + ?Sized;

    #[with]
    fn preimage<'a>(
        f: &'a Self,
        s: &'a <Self::Codomain as Measurable>::Subset<'a>,
    ) -> &'ref <Self::Domain as Measurable>::Subset<'a>
    where
        'subset: 'a;
}

// impl Measurable for () {
//     type Measure<R: Real> = R;

//     type PMeasure<R: Real> = ();

//     #[inline]
//     fn zero<R: Real>() -> Self::Measure<R>
//     where
//         Self::Measure<R>: Sized,
//     {
//         R::zero()
//     }

//     #[inline]
//     fn total<R: Real>(m: &Self::Measure<R>) -> R {
//         *m
//     }

//     #[inline]
//     fn normalize<R: Real>(m: &Self::Measure<R>) -> Option<Self::PMeasure<R>>
//     where
//         Self::PMeasure<R>: Sized,
//     {
//         R::normalize(&mut [*m])
//     }

//     #[inline]
//     fn with_normalized<R: Real, T>(
//         m: &Self::Measure<R>,
//         f: impl for<'a> FnOnce(&'a Self::PMeasure<R>) -> T,
//     ) -> Option<T> {
//         Some(f(&Self::normalize(m)?))
//     }
// }

// impl Measurable for bool {
//     // Measure values for [true, false].
//     type Measure<R: Real> = [R; 2];

//     // Probability of true.
//     type PMeasure<R: Real> = R;

//     #[inline]
//     fn zero<R: Real>() -> Self::Measure<R>
//     where
//         Self::Measure<R>: Sized,
//     {
//         [R::zero(), R::zero()]
//     }

//     #[inline]
//     fn total<R: Real>(m: &Self::Measure<R>) -> R {
//         m[0] + m[1]
//     }

//     #[inline]
//     fn normalize<R: Real>(m: &Self::Measure<R>) -> Option<Self::PMeasure<R>>
//     where
//         Self::PMeasure<R>: Sized,
//     {
//         Some(R::normalize_static(*m)?[0])
//     }

//     #[inline]
//     fn with_normalized<R: Real, T>(
//         m: &Self::Measure<R>,
//         f: impl for<'a> FnOnce(&'a Self::PMeasure<R>) -> T,
//     ) -> Option<T> {
//         Some(f(&Self::normalize(m)?))
//     }
// }

// impl<Re: Real> Measurable for Re {
//     type Measure<R: Real> = (R, dyn RealDistribution<R = R>);

//     type PMeasure<R: Real> = dyn RealDistribution<R = R>;

//     #[inline]
//     fn zero<R: Real>() -> Self::Measure<R>
//     where
//         Self::Measure<R>: Sized,
//     {
//         unreachable!()
//     }

//     #[inline]
//     fn total<R: Real>(m: &Self::Measure<R>) -> R {
//         m.0
//     }

//     fn normalize<R: Real>(_m: &Self::Measure<R>) -> Option<Self::PMeasure<R>>
//     where
//         Self::PMeasure<R>: Sized,
//     {
//         unreachable!()
//     }

//     #[inline]
//     fn with_normalized<R: Real, T>(
//         m: &Self::Measure<R>,
//         f: impl for<'a> FnOnce(&'a Self::PMeasure<R>) -> T,
//     ) -> Option<T> {
//         Some(f(&m.1))
//     }
// }

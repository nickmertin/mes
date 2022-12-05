use crate::real::{Real, RealDistribution};

/// Describes a data type as a measurable space.
pub trait Measurable {
    /// Describes an arbitrary finite measure over [`Self`].
    type Measure<R: Real>: ?Sized;

    /// Describes an arbitrary probability measure over [`Self`].
    type PMeasure<R: Real>: ?Sized;

    /// Produces the zero measure.
    fn zero<R: Real>() -> Self::Measure<R>
    where
        Self::Measure<R>: Sized;

    /// Computes the total measure.
    fn total<R: Real>(m: &Self::Measure<R>) -> R;

    /// Normalizes a finite measure into a probability measure. Returns [`None`]
    /// if the measure cannot be normalized (e.g., its total measure is zero).
    /// This can only be used when [`Self::PMeasure`] implements [`Sized`].
    fn normalize<R: Real>(m: &Self::Measure<R>) -> Option<Self::PMeasure<R>>
    where
        Self::PMeasure<R>: Sized;

    /// Normalizes a finite measure into a probability measure, and passes it to
    /// the given function `f`. Returns [`None`] and does not call `f` if
    /// the measure cannot be normalized (e.g., its total measure is zero).
    fn with_normalized<R: Real, T>(
        m: &Self::Measure<R>,
        f: impl for<'a> FnOnce(&'a Self::PMeasure<R>) -> T,
    ) -> Option<T>;
}

impl Measurable for () {
    type Measure<R: Real> = R;

    type PMeasure<R: Real> = ();

    #[inline]
    fn zero<R: Real>() -> Self::Measure<R>
    where
        Self::Measure<R>: Sized,
    {
        R::zero()
    }

    #[inline]
    fn total<R: Real>(m: &Self::Measure<R>) -> R {
        *m
    }

    #[inline]
    fn normalize<R: Real>(m: &Self::Measure<R>) -> Option<Self::PMeasure<R>>
    where
        Self::PMeasure<R>: Sized,
    {
        R::normalize(&mut [*m])
    }

    #[inline]
    fn with_normalized<R: Real, T>(
        m: &Self::Measure<R>,
        f: impl for<'a> FnOnce(&'a Self::PMeasure<R>) -> T,
    ) -> Option<T> {
        Some(f(&Self::normalize(m)?))
    }
}

impl Measurable for bool {
    // Measure values for [true, false].
    type Measure<R: Real> = [R; 2];

    // Probability of true.
    type PMeasure<R: Real> = R;

    #[inline]
    fn zero<R: Real>() -> Self::Measure<R>
    where
        Self::Measure<R>: Sized,
    {
        [R::zero(), R::zero()]
    }

    #[inline]
    fn total<R: Real>(m: &Self::Measure<R>) -> R {
        m[0] + m[1]
    }

    #[inline]
    fn normalize<R: Real>(m: &Self::Measure<R>) -> Option<Self::PMeasure<R>>
    where
        Self::PMeasure<R>: Sized,
    {
        Some(R::normalize_static(*m)?[0])
    }

    #[inline]
    fn with_normalized<R: Real, T>(
        m: &Self::Measure<R>,
        f: impl for<'a> FnOnce(&'a Self::PMeasure<R>) -> T,
    ) -> Option<T> {
        Some(f(&Self::normalize(m)?))
    }
}

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

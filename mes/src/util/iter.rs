use core::{
    marker::PhantomData,
    mem::{transmute, MaybeUninit},
};

use with_locals::with;

use super::{BasicLGType, LGType};

pub trait LocalIterator {
    type Item: LGType;

    // #[with]
    // fn next(&mut self) -> Option<&'ref Self::Item<'ref>>;

    fn next(&mut self) -> Option<&'_ <Self::Item as LGType>::Type<'_>>;

    fn map<
        'data,
        U: LGType + 'data,
        F: for<'a> Fn(&'a <Self::Item as LGType>::Type<'a>) -> U::Type<'a>,
    >(
        self,
        f: F,
    ) -> Map<'data, Self, U, F>
    where
        Self: Sized,
        for<'a> U::Type<'a>: Sized + Copy,
    {
        Map {
            base: self,
            f,
            value: MaybeUninit::uninit(),
        }
    }
}

pub struct Map<
    'data,
    I: LocalIterator,
    U: LGType + 'data,
    F: for<'a> Fn(&'a <I::Item as LGType>::Type<'a>) -> U::Type<'a>,
> where
    for<'a> U::Type<'a>: Sized + Copy,
{
    base: I,
    f: F,
    value: MaybeUninit<U::Type<'data>>,
}

impl<
        'data,
        I: LocalIterator,
        U: LGType + 'data,
        F: for<'a> Fn(&'a <I::Item as LGType>::Type<'a>) -> U::Type<'a>,
    > LocalIterator for Map<'data, I, U, F>
where
    for<'a> U::Type<'a>: Sized + Copy,
{
    type Item = U;

    // #[with]
    // fn next(&mut self) -> Option<&'ref Self::Item<'ref>> {}

    fn next<'a>(&'a mut self) -> Option<&'a <Self::Item as LGType>::Type<'a>> {
        union RefUnion<'a, 'data: 'a, U: LGType + 'data>
        where
            for<'b> <U as LGType>::Type<'b>: Sized,
        {
            static_ref: &'a mut MaybeUninit<U::Type<'data>>,
            local_ref: &'a mut MaybeUninit<U::Type<'a>>,
        }

        let value = unsafe {
            RefUnion::<'a, 'data, U> {
                static_ref: &mut self.value,
            }
            .local_ref
        };

        let r = self.base.next()?;
        let new_value = (self.f)(r);
        Some(value.write(new_value))
    }
}

pub struct RefMap<
    I: LocalIterator,
    U: LGType,
    F: for<'a> Fn(&'a <I::Item as LGType>::Type<'a>) -> &'a U::Type<'a>,
> {
    iterator: I,
    f: F,
    _phantom: PhantomData<U>,
}

impl<
        I: LocalIterator,
        U: LGType,
        F: for<'a> Fn(&'a <I::Item as LGType>::Type<'a>) -> &'a U::Type<'a>,
    > LocalIterator for RefMap<I, U, F>
{
    type Item = U;

    // #[with]
    // fn next(&mut self) -> Option<&'ref Self::Item<'ref>> {}

    fn next(&mut self) -> Option<&'_ <Self::Item as LGType>::Type<'_>> {
        let r = self.iterator.next()?;
        Some((self.f)(r))
    }
}

impl<I: Iterator> LocalIterator for I {
    type Item = BasicLGType<I::Item>;

    fn next(&mut self) -> Option<&'_ <Self::Item as LGType>::Type<'_>> {
        let value = Iterator::next(self)?;
        todo!()
    }
}

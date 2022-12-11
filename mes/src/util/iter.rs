use core::{
    marker::PhantomData,
    mem::{transmute, MaybeUninit},
};

use with_locals::with;

use super::{LGType, LGTypeCopy};

pub trait LocalIterator {
    type Item<'a>: ?Sized + 'a
    where
        Self: 'a;

    // #[with]
    // fn next(&mut self) -> Option<&'ref Self::Item<'ref>>;

    fn next(&mut self) -> Option<&'_ Self::Item<'_>>;

    fn map<U: LGTypeCopy, F: for<'a> Fn(&'a Self::Item<'a>) -> U::Type<'a>>(
        self,
        f: F,
    ) -> Map<Self, U, F>
    where
        Self: Sized,
    {
        Map {
            base: self,
            f,
            value: MaybeUninit::uninit(),
        }
    }
}

pub struct Map<I: LocalIterator, U: LGTypeCopy, F: for<'a> Fn(&'a I::Item<'a>) -> U::Type<'a>> {
    base: I,
    f: F,
    value: MaybeUninit<U::Type<'static>>,
}

impl<I: LocalIterator, U: LGTypeCopy, F: for<'a> Fn(&'a I::Item<'a>) -> U::Type<'a>> LocalIterator
    for Map<I, U, F>
{
    type Item<'a> = U::Type<'a>
    where
        Self: 'a;

    // #[with]
    // fn next(&mut self) -> Option<&'ref Self::Item<'ref>> {}

    fn next<'a>(&'a mut self) -> Option<&'a Self::Item<'a>> {
        union RefUnion<'a, U: LGTypeCopy> {
            static_ref: &'a mut MaybeUninit<U::Type<'static>>,
            local_ref: &'a mut MaybeUninit<U::Type<'a>>,
        }

        let value = unsafe {
            RefUnion::<'a, U> {
                static_ref: &mut self.value,
            }
            .local_ref
        };

        let r = self.base.next()?;
        let new_value = (self.f)(r);
        Some(value.write(new_value))
    }
}

pub struct RefMap<I: LocalIterator, U: LGType, F: for<'a> Fn(&'a I::Item<'a>) -> &'a U::Type<'a>> {
    iterator: I,
    f: F,
    _phantom: PhantomData<U>,
}

impl<I: LocalIterator, U: LGType, F: for<'a> Fn(&'a I::Item<'a>) -> &'a U::Type<'a>> LocalIterator
    for RefMap<I, U, F>
{
    type Item<'a> = U::Type<'a>
    where
        Self: 'a;

    // #[with]
    // fn next(&mut self) -> Option<&'ref Self::Item<'ref>> {}

    fn next(&mut self) -> Option<&'_ Self::Item<'_>> {
        let r = self.iterator.next()?;
        Some((self.f)(r))
    }
}

impl<I: Iterator> LocalIterator for I {
    type Item<'a> = I::Item
    where
        Self: 'a;

    fn next(&mut self) -> Option<&'_ Self::Item<'_>> {
        let value = Iterator::next(self)?;
        todo!()
    }
}

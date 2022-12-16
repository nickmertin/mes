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
}

pub trait LocalIteratorExt: LocalIterator {
    // fn any(mut self, f: impl for<'a> Fn(&'a <Self::Item as LGType>::Type<'a>) ->
    // bool) -> bool where
    //     Self: Sized,
    // {
    //     while let Some(value) = self.next() {
    //         if f(value) {
    //             return true;
    //         }
    //     }
    //     false
    // }

    // fn all(self, f: impl for<'a> Fn(&'a <Self::Item as LGType>::Type<'a>) ->
    // bool) -> bool where
    //     Self: Sized,
    // {
    //     !self.any(|x| !f(x))
    // }

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
        U::Type<'data>: Sized + Copy,
    {
        Map {
            base: self,
            f,
            value: MaybeUninit::uninit(),
        }
    }
}

impl<I: LocalIterator> LocalIteratorExt for I {}

#[macro_export]
macro_rules! any {
    ($x:pat in $i:expr => $y:expr) => {{
        let mut i = $i;
        let mut result = false;
        // let f = |$x| $y;
        while let ::core::option::Option::Some($x) = $crate::util::iter::LocalIterator::next(&mut i)
        {
            if $y {
                result = true;
                break;
            }
        }
        result
    }};
}

#[macro_export]
macro_rules! all {
    ($x:pat in $i:expr => $y:expr) => {!$crate::any!($x in $i => !$y)};
}

// #[macro_export]
// macro_rules! map {
//     ($x:pat in $i:expr => $y:expr) => {
//         $crate::util::iter::Map::new()
//     };
// }

pub struct Map<
    'data,
    I: LocalIterator,
    U: LGType + 'data,
    F: for<'a> Fn(&'a <I::Item as LGType>::Type<'a>) -> U::Type<'a>,
> where
    U::Type<'data>: Sized + Copy,
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
    > Map<'data, I, U, F>
where
    U::Type<'data>: Sized + Copy,
{
    pub fn new(base: I, f: F) -> Self {
        Self {
            base,
            f,
            value: MaybeUninit::uninit(),
        }
    }
}

impl<
        'data,
        I: LocalIterator,
        U: LGType + 'data,
        F: for<'a> Fn(&'a <I::Item as LGType>::Type<'a>) -> U::Type<'a>,
    > Clone for Map<'data, I, U, F>
where
    I: Clone,
    F: Clone,
    U::Type<'data>: Sized + Copy,
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            f: self.f.clone(),
            value: self.value.clone(),
        }
    }
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

// impl<I: Iterator> LocalIterator for I {
//     type Item = BasicLGType<I::Item>;

//     fn next(&mut self) -> Option<&'_ <Self::Item as LGType>::Type<'_>> {
//         let value = Iterator::next(self)?;
//         todo!()
//     }
// }

#[derive(Clone, Copy)]
pub struct LocalSliceIterator<'a, T> {
    slice: &'a [T],
}

impl<'a, T> LocalIterator for LocalSliceIterator<'a, T> {
    type Item = BasicLGType<'a, T>;

    fn next(&mut self) -> Option<&'_ <Self::Item as LGType>::Type<'_>> {
        let (head, tail) = self.slice.split_first()?;
        self.slice = tail;
        Some(head)
    }
}

pub trait SliceExt {
    type Item;

    fn local_iter(&self) -> LocalSliceIterator<'_, Self::Item>;
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn local_iter(&self) -> LocalSliceIterator<'_, Self::Item> {
        LocalSliceIterator { slice: self }
    }
}

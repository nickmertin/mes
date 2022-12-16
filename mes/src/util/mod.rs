use core::marker::PhantomData;

pub mod iter;
pub mod proxy;

// pub enum LGTypeCategory {
//     BASIC,
//     SIZED,
//     COPY,
// }

// pub trait LGTypeBase {
//     const CATEGORY: LGTypeCategory;
// }

pub trait LGType {
    type Type<'a>: ?Sized + 'a
    where
        Self: 'a;
}

// pub trait LGTypeSized: LGType
// where
//     for<'a> Self::Type<'a>: Sized,
// {
//     // type Type<'a>: 'a;
// }

// pub trait LGTypeCopy: LGType
// where
//     for<'a> Self::Type<'a>: Sized + Copy,
// {
//     // type Type<'a>: Copy + 'a;
// }

// impl<T: LGTypeSized> LGType for T {
//     type Type<'a> = <Self as LGTypeSized>::Type<'a>;
// }

pub struct BasicLGType<'scope, T: ?Sized + 'scope>(PhantomData<&'scope T>);

impl<'scope, T: ?Sized + 'scope> LGType for BasicLGType<'scope, T> {
    type Type<'a> = T where Self: 'a;
}

// impl<T: 'static> LGTypeSized for BasicLGType<T> {
//     type Type<'a> = T;
// }

// impl<T: Copy + 'static> LGTypeCopy for BasicLGType<T> {
//     type Type<'a> = T;
// }

// pub struct LGTypeUpcast<U>(Invariant<U>);

// impl<U: LGTypeSized> LGType for LGTypeUpcast<U> {
//     type Type<'a> = U::Type<'a>;
// }
// impl<U: LGTypeCopy> LGType for LGTypeUpcast<U> {
//     type Type<'a> = U::Type<'a>;
// }

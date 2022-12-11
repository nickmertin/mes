use type_variance::Invariant;

pub mod iter;
pub mod proxy;

pub trait LGType {
    type Type<'a>: ?Sized + 'a;
}

pub trait LGTypeSized {
    type Type<'a>: 'a;
}
pub trait LGTypeCopy {
    type Type<'a>: Copy + 'a;
}

// impl<T: LGTypeSized> LGType for T {
//     type Type<'a> = <Self as LGTypeSized>::Type<'a>;
// }

pub struct BasicLGType<T: ?Sized + 'static>(Invariant<T>);

impl<T: ?Sized + 'static> LGType for BasicLGType<T> {
    type Type<'a> = T;
}

impl<T: 'static> LGTypeSized for BasicLGType<T> {
    type Type<'a> = T;
}
impl<T: Copy + 'static> LGTypeCopy for BasicLGType<T> {
    type Type<'a> = T;
}

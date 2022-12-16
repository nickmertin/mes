//! Miscellaneous utilities.

use core::marker::PhantomData;
use with_locals::with;

/// Provides limited access to a value of type `T`.
pub struct Proxy<'a, T: ?Sized>(ProxyState<'a, T>);

impl<'a, T: ?Sized> Clone for Proxy<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized> Copy for Proxy<'a, T> {}

pub type AccessorFn<'a, T> = dyn for<'b> Fn(&'b (dyn for<'c> FnMut(&'c T) + 'b));
// pub type MappingFn<'a, T, U> = dyn for<'b> Fn(&'b )

enum ProxyState<'a, T: ?Sized> {
    ProxyRef(&'a T),
    ProxyFn(&'a AccessorFn<'a, T>),
    ProxyMap(&'a (dyn MapFn<T> + 'a), *const ()),
}

impl<'a, T: ?Sized> Clone for ProxyState<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized> Copy for ProxyState<'a, T> {}

impl<'a, T: ?Sized> Proxy<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Self(ProxyState::ProxyRef(value))
    }

    pub fn new_fn(accessor: &'a AccessorFn<'a, T>) -> Self {
        Self(ProxyState::ProxyFn(accessor))
    }

    #[allow(unused_macros)]
    #[with(continuation_name = ret)]
    pub fn access(&self) -> &'ref T {
        let mut ret = Some(ret);
        let mut result = None;
        let cont: &mut (dyn FnMut(&T) + '_) =
            &mut |x| result = Some(ret.take().expect("continuation called multiple times!")(x));

        match self.0 {
            ProxyState::ProxyRef(value) => cont(value),
            ProxyState::ProxyFn(accessor) => accessor(cont),
            ProxyState::ProxyMap(f, target) => unsafe { f.eval(target, cont) },
        }

        result.expect("continuation was never called!")
    }

    // #[allow(unused_macros)]
    // #[with(continuation_name = ret)]
    // pub fn map<U: ?Sized>(
    //     self,
    //     f: impl Fn(&T, &(dyn FnMut(&U) + 'a)) + 'a,
    // ) -> &'ref Proxy<'ref, U> {
    //     match self.0 {
    //         ProxyState::ProxyRef(value) => {
    //             let mut result = None;
    //             f(value, &|x| result = Some(ret(&Proxy::new(x))));
    //             result.unwrap()
    //         }
    //         ProxyState::ProxyFn(accessor) => {
    //             let new_accessor = |g| accessor(&|x| f(x, g));
    //             ret(&Proxy::new_fn(&new_accessor))
    //         }
    //         ProxyState::ProxyMap(g, target) => {
    //             todo!()
    //         }
    //     }
    // }

    pub fn map<'b, U: ?Sized>(
        &'b self,
        f: &'b (impl for<'c> Fn(&'c T, &'c mut (dyn for<'d> FnMut(&'d U) + 'c)) + 'b),
    ) -> Proxy<'b, U> {
        #[repr(transparent)]
        struct FnWrapper<
            T: ?Sized,
            U: ?Sized,
            F: for<'c> Fn(&'c T, &'c mut (dyn for<'d> FnMut(&'d U) + 'c)) + ?Sized,
        >(
            PhantomData<fn(&T) -> &U>,
            F,
            // Contravariant<T>,
            // Covariant<U>,
        );

        impl<
                'a,
                T: ?Sized + 'a,
                U: ?Sized,
                F: for<'c> Fn(&'c T, &'c mut (dyn for<'d> FnMut(&'d U) + 'c)) + ?Sized,
            > MapFn<U> for FnWrapper<T, U, F>
        {
            #[with]
            unsafe fn eval(&self, target: *const (), f: &mut (dyn FnMut(&U) + '_)) {
                let target = &*(target as *const Proxy<'a, T>);
                let x: &'ref _ = target.access();
                self.1(x, f);
            }
        }

        union FnUnion<
            'a,
            T: ?Sized,
            U: ?Sized,
            F: for<'c> Fn(&'c T, &'c mut (dyn for<'d> FnMut(&'d U) + 'c)) + ?Sized,
        > {
            f: &'a F,
            wrapper: &'a FnWrapper<T, U, F>,
        }

        Proxy(ProxyState::ProxyMap(
            unsafe { FnUnion { f }.wrapper },
            self as *const _ as _,
        ))
    }
}

trait MapFn<T: ?Sized> {
    unsafe fn eval(&self, target: *const (), f: &mut (dyn FnMut(&T) + '_));
}

// pub struct ProxyAsLGType<'context, T: ?Sized +
// 'context>(PhantomData<&'context T>);

// impl<'context, T> LGType for ProxyAsLGType<'context, T> {
//     type Type<'a> = Proxy<'a, T>
//     where
//         Self: 'a;
// }

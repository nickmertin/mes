use darling::{ast::Fields, FromField, FromVariant};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Index, Type};

use super::Input;

#[derive(Clone, FromVariant)]
#[darling(supports(newtype, unit))]
pub struct Variant {
    fields: Fields<VariantField>,
}

#[derive(Clone, FromField)]
pub struct VariantField {
    ty: Type,
}

pub(super) fn derive(input: Input) -> TokenStream {
    let r_ident = input.r_ident();
    let Input { data, .. } = input;
    let data = data.take_enum().unwrap();
    let len = data.len();

    if len == 0 {
        return empty(r_ident);
    }

    let len_m1 = len - 1;
    let real = quote!(::mes::real::Real);
    let sized = quote!(::core::marker::Sized);
    let option = quote!(::core::option::Option);
    let fn_once = quote!(::core::ops::FnOnce);

    let types = data
        .into_iter()
        .map(|v| {
            if let Some(VariantField { ty }) = v.fields.into_iter().next() {
                quote!(#ty)
            } else {
                quote!(())
            }
        })
        .collect_vec();

    let indices = (0..len).into_iter().map(Index::from).collect_vec();

    quote! {
        type Measure<#r_ident: #real> = (#(<#types as Measurable>::Measure<#r_ident>,)*);

        type PMeasure<#r_ident: #real> = ([#r_ident; #len_m1], #(#option<<#types as Measurable>::PMeasure<#r_ident>>),*);

        fn total<R: #real>(m: &Self::Measure<#r_ident>) -> #r_ident {
            (#(<#types as Measurable>::total(&m.#indices))+*)
        }

        fn normalize<#r_ident: #real>(m: &Self::Measure<#r_ident>) -> #option<Self::PMeasure<#r_ident>>
        where
            Self::PMeasure<#r_ident>: #sized,
        {
            let mut probabilities: [#r_ident; #len] = [#(<#types as Measurable>::total(&m.#indices)),*];
            <#r_ident as #real>::normalize(&mut probabilities)?;
            #option::Some((probabilities[..#len_m1].try_into().ok()?, #(<#types as Measurable>::normalize(&m.#indices)),*))
        }

        fn with_normalized<#r_ident: #real, T>(
            m: &Self::Measure<#r_ident>,
            f: impl for<'a> #fn_once(&'a Self::PMeasure<#r_ident>) -> T,
        ) -> Option<T> {
            #option::Some(f(&<Self as Measurable>::normalize(m)?))
        }
    }
}

fn empty(r_ident: Ident) -> TokenStream {
    let real = quote!(::mes::real::Real);
    let sized = quote!(::core::marker::Sized);
    let option = quote!(::core::option::Option);
    let fn_once = quote!(::core::ops::FnOnce);

    quote! {
        type Measure<#r_ident: #real> = ();

        type PMeasure<#r_ident: #real> = ::mes::void::Void;

        #[inline]
        fn normalize<#r_ident: #real>(_m: &Self::Measure<#r_ident>) -> #option<Self::PMeasure<#r_ident>>
        where
            Self::PMeasure<#r_ident>: #sized,
        {
            #option::None
        }

        #[inline]
        fn with_normalized<#r_ident: #real, T>(
            m: &Self::Measure<#r_ident>,
            f: impl for<'a> #fn_once(&'a Self::PMeasure<#r_ident>) -> T,
        ) -> Option<T> {
            #option::None
        }
    }
    .into()
}

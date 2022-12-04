use proc_macro2::TokenStream;
use quote::quote;
use syn::{ConstParam, GenericParam, Generics, LifetimeDef, TypeParam};

pub(crate) fn as_arguments(generics: &Generics) -> TokenStream {
    if let Some((lt, gt)) = generics
        .lt_token
        .iter()
        .zip(generics.gt_token.iter())
        .next()
    {
        let params = generics.params.pairs().map(|p| {
            let c = p.punct().into_iter();
            match p.value() {
                GenericParam::Type(TypeParam { ident, .. }) => quote!(#ident #(#c)*),
                GenericParam::Lifetime(LifetimeDef { lifetime, .. }) => quote!(#lifetime #(#c)*),
                GenericParam::Const(ConstParam { ident, .. }) => quote!(#ident #(#c)*),
            }
        });

        quote!(#lt #(#params)* #gt)
    } else {
        quote!()
    }
}

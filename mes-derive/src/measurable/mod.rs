use darling::{
    ast::{Data, GenericParamExt},
    FromDeriveInput,
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Generics, Ident};

mod for_enum;

#[derive(Clone, FromDeriveInput)]
#[darling(supports(enum_any))]
struct Input {
    ident: Ident,
    data: Data<for_enum::Variant, ()>,
    generics: Generics,
}

impl Input {
    fn r_ident(&self) -> Ident {
        let mut ident = "R".to_string();
        while self
            .generics
            .params
            .iter()
            .filter_map(GenericParamExt::as_type_param)
            .any(|p| p.ident == ident)
        {
            ident += "R";
        }

        Ident::new(ident.as_str(), Span::call_site())
    }
}

pub fn derive(input: TokenStream) -> TokenStream {
    let input = Input::from_derive_input(&(parse_macro_input!(input))).unwrap();
    for_enum::derive(input).into()
}

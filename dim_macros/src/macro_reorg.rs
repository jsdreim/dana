use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    Result,
    Token,
};
use crate::unit_def::*;


#[derive(Debug)]
pub struct UnitMapping<U: UnitValid> {
    type_base: UnitDef<U>,
    types: Vec<UnitDef<U>>,
}

impl<U: UnitValid> Parse for UnitMapping<U> {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![impl]>()?;
        let type_base = input.parse::<UnitDef<U>>()?;
        let mut types = Vec::new();

        let inner;
        braced!(inner in input);

        while inner.parse::<Token![as]>().is_ok() {
            types.push(inner.parse::<UnitDef<U>>()?);
            inner.parse::<Token![;]>()?;
        }

        Ok(Self { type_base, types })
    }
}


struct ImplReorg<'p, U: UnitValid> {
    params: &'p Vec<syn::TypeParam>,
    from: &'p UnitDef<U>,
    into: &'p UnitDef<U>,
}

impl<U: UnitValid> ImplReorg<'_, U> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.from, &mut self.into);
    }
}

impl<U: UnitValid> ToTokens for ImplReorg<'_, U> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let type_from = self.from.as_type();
        let type_into = self.into.as_type();
        let bind_from = self.from.as_value();
        let unit_into = self.into.as_value();

        tokens.extend(quote!(impl));

        if !self.params.is_empty() {
            tokens.extend(quote!(<));

            for param in self.params {
                tokens.extend(quote!(#param,));
            }

            tokens.extend(quote!(>));
        }

        tokens.extend(quote! {
            ::dimensional::units::traits::transform::Simplify<#type_into> for #type_from {
                fn simplify<__S: crate::Scalar>(self)
                    -> ::dimensional::units::traits::transform::Conversion<#type_into, __S>
                {
                    #[allow(non_snake_case)]
                    let #bind_from = self;
                    ::dimensional::units::traits::transform::Conversion::units(#unit_into)
                }
            }
        });

        // tokens.extend(quote! {
        //     ConvertFrom<#from> for #into {}
        // });
    }
}


// #[derive(Debug)]
pub struct Reorg {
    pub params: Vec<syn::TypeParam>,
    pub maps: Vec<UnitMapping<syn::Ident>>,
}

impl Parse for Reorg {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut params = Vec::new();
        let mut maps = Vec::new();

        if input.parse::<Token![where]>().is_ok() {
            while let Ok(param) = input.parse::<syn::TypeParam>() {
                params.push(param);

                if input.parse::<Token![,]>().is_err() {
                    break;
                }
            }

            input.parse::<Token![;]>()?;
        }

        while let Ok(conv) = input.parse::<UnitMapping<_>>() {
            maps.push(conv);
        }

        Ok(Self { params, maps })
    }
}

impl ToTokens for Reorg {
    //noinspection RsUnusedMut
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut impls: usize = 0;

        macro_rules! implement {
            ($ts:expr, $from:expr, $into:expr) => {
                let mut impl_reorg = ImplReorg {
                    params: &self.params,
                    from: $from,
                    into: $into,
                };

                $ts.extend(quote!(#impl_reorg));
                impls += 1;

                impl_reorg.swap();

                $ts.extend(quote!(#impl_reorg));
                impls += 1;
            };
        }

        for map in &self.maps {
            for reorg in &map.types {
                implement!(tokens, &map.type_base, reorg);
            }

            let i_stop = map.types.len();
            let mut i_from = 0;

            while i_from < i_stop {
                let from = &map.types[i_from];
                let mut i_into = i_from + 1;

                while i_into < i_stop {
                    let into = &map.types[i_into];

                    implement!(tokens, from, into);

                    i_into += 1;
                }

                i_from += 1;
            }
        }

        eprintln!("Implementations: {impls}");
    }
}

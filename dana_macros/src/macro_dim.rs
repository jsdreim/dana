use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse::{Parse, ParseStream}, Result, Token};
use crate::util::{PathSep, typenum_int};


//region Sequence-based definition.
#[derive(Debug, Default)]
pub struct DimExp {
    sum: i32,
    spans: Vec<Span>,
}

impl DimExp {
    pub const fn new(sum: i32) -> Self {
        Self { sum, spans: Vec::new() }
    }

    pub fn label(&self) -> String {
        typenum_int(self.sum)
    }
}

impl Parse for DimExp {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.parse::<Token![^]>().is_ok() {
            let literal: syn::LitInt = input.parse()?;

            Ok(Self {
                sum: literal.base10_parse()?,
                spans: vec![literal.span()],
            })
        } else {
            Ok(Self::new(1))
        }
    }
}

impl ToTokens for DimExp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = match self.spans.first() {
            Some(span) => *span,
            None => Span::call_site(),
        };

        let ident = syn::Ident::new(&self.label(), span);
        tokens.extend(quote!(::typenum::consts::#ident));
    }
}


#[derive(Debug, Default)]
pub struct DimSeq {
    pub exp_l: DimExp,
    pub exp_m: DimExp,
    pub exp_t: DimExp,
    pub exp_i: DimExp,
    pub exp_k: DimExp,
    pub exp_n: DimExp,
    pub exp_j: DimExp,
}

impl Parse for DimSeq {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![<]>()?;

        let mut total = Self::default();

        let mut i = 0;
        let array = [
            &mut total.exp_l,
            &mut total.exp_m,
            &mut total.exp_t,
            &mut total.exp_i,
            &mut total.exp_k,
            &mut total.exp_n,
            &mut total.exp_j,
        ];

        while let Ok(literal) = input.parse::<syn::LitInt>() {
            if array.len() <= i {
                return Err(syn::Error::new(
                    literal.span(),
                    &format!("too many dimensions specified, expected at \
                    most {}", array.len()),
                    // "too many dimensions specified",
                ));
            }

            let exp: i32 = literal.base10_parse()?;
            array[i].sum += exp;
            array[i].spans.push(literal.span());
            i += 1;

            if input.parse::<Token![,]>().is_err() {
                break;
            }
        }

        input.parse::<Token![>]>()?;

        Ok(total)
    }
}

impl ToTokens for DimSeq {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { exp_l, exp_m, exp_t, exp_i, exp_k, exp_n, exp_j } = self;

        tokens.extend(quote! {
            ::dana::dimension::Dimension<
                #exp_l, #exp_m, #exp_t, #exp_i, #exp_k, #exp_n, #exp_j,
            >
        });
    }
}
//endregion


//region Product-based definition.
/// Literal `1`, single identifier, or path.
#[derive(Debug)]
pub enum DimBase {
    /// A literal `1`.
    One(Span),
    /// One identifier.
    Ident(syn::Ident),
    /// One token, possibly preceded by a path separator, followed by a sequence
    ///     of path separators and identifiers.
    Path { lead: Option<PathSep>, first: TokenTree, path: Vec<syn::Ident> },
}

impl Parse for DimBase {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Err(input.error("expected `1` or dimension"))
        } else if let Ok(literal) = input.parse::<syn::LitInt>() {
            let value: i32 = literal.base10_parse()?;

            if value == 1 {
                Ok(Self::One(literal.span()))
            } else {
                Err(syn::Error::new(
                    literal.span(),
                    &format!("expected `1` or dimension, found `{literal}`"),
                ))
            }
        } else {
            let lead = input.parse().ok();

            if lead.is_some() || input.peek2(Token![::]) {
                let mut path = Vec::new();

                let first = input.parse()?;

                while input.parse::<Token![::]>().is_ok() {
                    path.push(input.parse()?);
                }

                Ok(Self::Path { lead, first, path })
            } else {
                Ok(Self::Ident(input.parse()?))
            }
        }
    }
}

impl ToTokens for DimBase {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            &Self::One(span) => {
                tokens.extend(quote_spanned!(span=> ::dana::dimension::One))
            }
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Path { lead, first, path } => {
                tokens.extend(quote!(#lead #first #(::#path)*));
            }
        }
    }
}


#[derive(Debug)]
pub struct DimPow {
    base: DimBase,
    exp: Option<(i32, Span)>,
}

impl Parse for DimPow {
    fn parse(input: ParseStream) -> Result<Self> {
        let base = input.parse()?;

        let exp = if input.parse::<Token![^]>().is_ok() {
            let literal = input.parse::<syn::LitInt>()?;
            let value = literal.base10_parse()?;

            Some((value, literal.span()))
        } else {
            None
        };

        Ok(Self { base, exp })
    }
}

impl ToTokens for DimPow {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut base = self.base.to_token_stream();

        if let Some((value, span)) = &self.exp {
            let label = typenum_int(*value);
            let ident = syn::Ident::new(&label, *span);

            base = quote!(
                <#base as ::dana::dimension::DimPowType<::typenum::#ident>>::Output
            );
        }

        base.to_tokens(tokens);
    }
}


#[derive(Debug)]
pub struct DimSpec {
    factors: Vec<DimPow>,
}

impl Parse for DimSpec {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut factors = Vec::new();

        if let Ok(dim) = input.parse() {
            factors.push(dim);

            while input.parse::<Token![*]>().is_ok() {
                factors.push(input.parse()?);
            }
        }

        Ok(Self { factors })
    }
}

impl ToTokens for DimSpec {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut iter = self.factors.iter();

        match iter.next() {
            Some(first) => {
                let mut dim = first.to_token_stream();

                for mul in iter {
                    dim = quote!(<#dim as ::core::ops::Mul<#mul>>::Output);
                }

                tokens.extend(dim);
            }
            None => {
                tokens.extend(quote!(::dana::dimension::One));
            }
        }
    }
}
//endregion


pub struct MacroDim {
    seq: Option<DimSeq>,
    mul: Option<DimSpec>,
}

impl Parse for MacroDim {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Err(input.error("expected `<`, `1`, or dimension"))
        } else {
            let seq = if input.peek(Token![<]) {
                Some(input.parse()?)
            } else {
                None
            };

            let mul = match &seq {
                Some(_) if input.parse::<Token![*]>().is_err() => None,
                __ => Some(input.parse()?),
            };

            Ok(Self { seq, mul })
        }
    }
}

impl ToTokens for MacroDim {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match (&self.seq, &self.mul) {
            (Some(seq), Some(mul)) => tokens.extend(quote!(
                <#seq as ::core::ops::Mul<#mul>>::Output
            )),
            (Some(seq), None) => seq.to_tokens(tokens),
            (None, Some(mul)) => mul.to_tokens(tokens),
            (None, None) => tokens.extend(quote!(::dana::dimension::One)),
        }
    }
}

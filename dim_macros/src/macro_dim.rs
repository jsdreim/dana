use std::cmp::Ordering;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, Result, Token};


#[derive(Debug, Default)]
pub struct DimExp {
    sum: i32,
    spans: Vec<Span>,
}

impl DimExp {
    pub const fn new(sum: i32) -> Self {
        Self { sum, spans: Vec::new() }
    }

    pub fn prefix(&self) -> char {
        match self.sum.cmp(&0) {
            Ordering::Less => 'N',
            Ordering::Equal => 'Z',
            Ordering::Greater => 'P',
        }
    }

    pub fn label(&self) -> String {
        format!("{}{}", self.prefix(), self.sum.abs())
    }

    pub fn add(&mut self, exp: i32, span: Span) {
        self.sum += exp;
        self.spans.push(span);
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
pub struct MacroDim {
    pub exp_l: DimExp,
    pub exp_m: DimExp,
    pub exp_t: DimExp,
    pub exp_i: DimExp,
    pub exp_k: DimExp,
    pub exp_n: DimExp,
    pub exp_j: DimExp,
}

impl Parse for MacroDim {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut total = Self::default();
        let mut first = true;

        while first || input.parse::<Token![*]>().is_ok() {
            first = false;

            if let Ok(literal) = input.parse::<syn::LitInt>() {
                let value: i32 = literal.base10_parse()?;

                if value != 1 {
                    return Err(syn::Error::new(
                        literal.span(),
                        &format!("expected `1` or dimension, found `{literal}`"),
                    ));
                }
            } else if let Ok(ident) = input.parse::<syn::Ident>() {
                let exp_new: DimExp = input.parse()?;

                let span_ident = ident.span();
                let span_exp = match exp_new.spans.first() {
                    Some(span) => match span_ident.join(*span) {
                        Some(joined) => joined,
                        None => *span,
                    }
                    None => span_ident,
                };

                let exp = match ident.to_string().to_lowercase().as_str() {
                    "l" => &mut total.exp_l,
                    "m" => &mut total.exp_m,
                    "t" => &mut total.exp_t,
                    "i" => &mut total.exp_i,
                    "θ" => &mut total.exp_k,
                    "k" => &mut total.exp_k,
                    "n" => &mut total.exp_n,
                    "j" => &mut total.exp_j,
                    _ => return Err(syn::Error::new(
                        span_ident,
                        &format!("unknown dimension: expected `L`, `M`, `T`, \
                        `I`, `Θ`, `K`, `N`, or `J`, found `{ident}`"),
                        // "unknown dimension",
                    )),
                };

                exp.add(exp_new.sum, span_exp);
            } else {
                if input.is_empty() {
                    return Err(input.error("expected `1` or dimension"));
                } else {
                    return Err(input.error("unexpected token, expected `1` or dimension"));
                }
            }
        }

        Ok(total)
    }
}

impl ToTokens for MacroDim {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { exp_l, exp_m, exp_t, exp_i, exp_k, exp_n, exp_j } = self;

        tokens.extend(quote! {
            ::dimensional::dimension::Dimension<
                #exp_l, #exp_m, #exp_t, #exp_i, #exp_k, #exp_n, #exp_j,
            >
        });
    }
}

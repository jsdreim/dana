use std::cmp::Ordering;
use num_traits::{PrimInt, Signed};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    Result,
    Token,
};


pub fn typenum_int<N: std::fmt::Display + PrimInt + Signed>(n: N) -> String {
    let prefix = match n.cmp(&N::zero()) {
        Ordering::Less => 'N',
        Ordering::Equal => 'Z',
        Ordering::Greater => 'P',
    };

    format!("{prefix}{}", n.abs())
}


// pub fn typenum_uint<N: std::fmt::Display + PrimInt + Unsigned>(n: N) -> String {
//     format!("U{n}")
// }


#[derive(Debug)]
pub struct PathSep { pub spans: [proc_macro2::Span; 2] }

impl Parse for PathSep {
    fn parse(input: ParseStream) -> Result<Self> {
        let syn::token::PathSep { spans } = input.parse()?;
        Ok(Self { spans })
    }
}

impl ToTokens for PathSep {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        syn::token::PathSep { spans: self.spans }.to_tokens(tokens)
    }
}


#[derive(Clone, Copy)]
pub enum Sign {
    Negative(Token![-]),
    Positive(Token![+]),
}

impl std::fmt::Debug for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Negative(_) => f.write_str("Negative"),
            Self::Positive(_) => f.write_str("Positive"),
        }
    }
}

impl Parse for Sign {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(t) = input.parse::<Token![-]>() {
            Ok(Self::Negative(t))
        } else if let Ok(t) = input.parse::<Token![+]>() {
            Ok(Self::Positive(t))
        } else {
            Err(input.error("expected '+' or '-'"))
        }
    }
}

impl ToTokens for Sign {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Negative(t) => t.to_tokens(tokens),
            Self::Positive(t) => t.to_tokens(tokens),
        }
    }
}

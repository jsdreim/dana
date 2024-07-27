use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    Result,
    Token,
};
use crate::unit_spec::UnitSpecType;


pub enum ScalarType {
    Inferred(Token![_]),
    Explicit(syn::Type),
}

impl std::fmt::Debug for ScalarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inferred(_) => f.write_str("Inferred(_)"),
            Self::Explicit(t) => write!(f, "Explicit({})", t.to_token_stream()),
        }
    }
}

impl Parse for ScalarType {
    fn parse(input: ParseStream) -> Result<Self> {
        match input.parse() {
            Ok(discard) => Ok(Self::Inferred(discard)),
            Err(_) => Ok(Self::Explicit(input.parse()?)),
        }
    }
}

impl ToTokens for ScalarType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Inferred(inner) => inner.to_tokens(tokens),
            Self::Explicit(inner) => inner.to_tokens(tokens),
        }
    }
}


#[derive(Debug)]
pub struct MacroQType {
    pub scalar: Option<ScalarType>,
    pub dim: UnitSpecType,
}

impl Parse for MacroQType {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();

        let scalar = match fork.parse() {
            Ok(scalar) if fork.parse::<Token![;]>().is_ok() => {
                input.advance_to(&fork);
                Some(scalar)
            }
            _ => None,
        };

        Ok(Self { scalar, dim: input.parse()? })
    }
}

impl ToTokens for MacroQType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dim = self.dim.as_type();

        tokens.extend(match &self.scalar {
            Some(scalar) => quote!(::dana::Quantity<#dim, #scalar>),
            None => quote!(::dana::Quantity<#dim>),
        });
    }
}

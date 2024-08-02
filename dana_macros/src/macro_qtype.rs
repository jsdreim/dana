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
        if input.is_empty() {
            Err(input.error("expected `_` or numeric type"))
        } else {
            match input.parse() {
                Ok(discard) => Ok(Self::Inferred(discard)),
                Err(_) => Ok(Self::Explicit(input.parse()?)),
            }
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

        //  Check for a valid scalar type before the unit specifier.
        let scalar_first = match fork.parse::<ScalarType>() {
            Ok(scalar) if parse_any!(fork, ;, as) => {
                //  Found one, and then also found a valid separator.
                input.advance_to(&fork);
                Some(scalar)
            }
            _ => None,
        };

        //  Parse the unit specifier.
        let dim = input.parse::<UnitSpecType>()?;

        //  Decide what to do for a scalar type.
        let scalar = if scalar_first.is_none() // Did not set one earlier?
            && input.parse::<Token![,]>().is_ok() // Also found a comma here?
        {
            //  Parse a scalar type now.
            Some(input.parse::<ScalarType>()?)
        } else {
            //  Reuse whatever was or was not found earlier.
            scalar_first
        };

        Ok(Self { scalar, dim })
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

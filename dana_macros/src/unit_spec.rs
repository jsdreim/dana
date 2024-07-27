use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Result,
    Token,
};
use crate::util::{PathSep, typenum_int};


/// Unit specifier ultimately destined to be used as an expression.
pub type UnitSpecExpr = UnitSpec<UnitCoreExpr>;

/// Unit specifier ultimately destined to be used as a type.
pub type UnitSpecType = UnitSpec<UnitCoreType>;


/// Marker trait indicating that a type is usable at the core of a [`UnitSpec`].
pub trait UnitCore: std::fmt::Debug + Parse + ToTokens {}
impl<T: std::fmt::Debug + Parse + ToTokens> UnitCore for T {}


/// Core type for an expression-destined unit specifier.
#[derive(Debug)]
pub enum UnitCoreExpr {
    /// One identifier.
    Ident(syn::Ident),
    /// One token, followed by a sequence of dots and field identifiers.
    Field { first: TokenTree, fields: Vec<syn::Ident> },
    /// One token, possibly preceded by a path separator, followed by a sequence
    ///     of path separators and identifiers.
    Path {
        lead: Option<PathSep>,
        first: TokenTree,
        path: Vec<syn::Ident>,
        fields: Vec<syn::Ident>,
    },
}

impl Parse for UnitCoreExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let lead = input.parse().ok();

        if lead.is_some() || input.peek2(Token![::]) {
            let mut path = Vec::new();
            let mut fields = Vec::new();

            let first = input.parse()?;

            while input.parse::<Token![::]>().is_ok() {
                path.push(input.parse()?);
            }

            while input.parse::<Token![.]>().is_ok() {
                fields.push(input.parse()?);
            }

            Ok(Self::Path { lead, first, path, fields })
        } else if input.peek2(Token![.]) {
            let mut fields = Vec::new();

            let first = input.parse()?;

            while input.parse::<Token![.]>().is_ok() {
                fields.push(input.parse()?);
            }

            Ok(Self::Field { first, fields })
        } else {
            Ok(Self::Ident(input.parse()?))
        }
    }
}

impl ToTokens for UnitCoreExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Field { first, fields } => {
                tokens.extend(quote!(#first #(.#fields)*));
            }
            Self::Path { lead, first, path, fields } => {
                tokens.extend(quote!(#lead #first #(::#path)* #(.#fields)*));
            }
        }
    }
}


/// Core type for an expression-destined unit specifier.
#[derive(Debug)]
pub enum UnitCoreType {
    /// One identifier.
    Ident(syn::Ident),
    /// One token, possibly preceded by a path separator, followed by a sequence
    ///     of path separators and identifiers.
    Path { lead: Option<PathSep>, first: TokenTree, path: Vec<syn::Ident> },
}

impl Parse for UnitCoreType {
    fn parse(input: ParseStream) -> Result<Self> {
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

impl ToTokens for UnitCoreType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Path { lead, first, path } => {
                tokens.extend(quote!(#lead #first #(::#path)*));
            }
        }
    }
}


pub enum Exponent {
    Whole(syn::LitInt),
    Frac(syn::LitInt, syn::LitInt),
}

impl std::fmt::Debug for Exponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Whole(a) => write!(f, "Whole({a})"),
            Self::Frac(a, b) => write!(f, "Frac({a}, {b})"),
        }
    }
}

impl Exponent {
    fn numerator(&self) -> &syn::LitInt {
        match self {
            Self::Whole(a) => a,
            Self::Frac(a, _) => a,
        }
    }

    fn denominator(&self) -> Option<&syn::LitInt> {
        match self {
            Self::Whole(_) => None,
            Self::Frac(_, b) => Some(b),
        }
    }
}

impl Parse for Exponent {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::token::Paren) {
            let inner;
            parenthesized!(inner in input);

            let first = inner.parse()?;

            if inner.is_empty() {
                Ok(Self::Whole(first))
            } else {
                inner.parse::<Token![/]>()?;
                Ok(Self::Frac(first, inner.parse()?))
            }
        } else {
            Ok(Self::Whole(input.parse()?))
        }
    }
}

impl ToTokens for Exponent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (a, _) = match self {
            Self::Whole(a) => (a, syn::LitInt::new("1", a.span())),
            Self::Frac(a, b) => (a, b.clone()),
        };

        match a.base10_parse::<i32>() {
            Ok(n) => {
                let ident = syn::Ident::new(&typenum_int(n), a.span());
                tokens.extend(quote!(::typenum::consts::#ident));
            }
            Err(e) => e.to_compile_error().to_tokens(tokens),
        }

        // tokens.extend(quote!(TypeFrac<#a, #b>));
    }
}


#[derive(Debug)]
struct UnitExp<U: UnitCore> {
    base: UnitSpec<U>,
    inv: bool,
    neg: bool,
    exp: Option<Exponent>,
}

impl<U: UnitCore> Parse for UnitExp<U> {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Err(input.error("expected unit"));
        }

        let inv = if let Ok(literal) = input.parse::<proc_macro2::Literal>() {
            //  Found a literal. The only literal allowed here is `1`, and it
            //      must be followed by `/` to specify an inverted unit.

            let lit_1 = literal.to_string() == "1";
            let div_follows = input.parse::<Token![/]>().is_ok();

            if lit_1 && div_follows {
                true
            } else {
                return Err(syn::Error::new(
                    literal.span(),
                    "expected unit or `1` divided by unit",
                ));
            }
        } else {
            false
        };

        let base = match input.parse() {
            Ok(unit) => UnitSpec::Leaf(unit),
            Err(_) => if input.peek(syn::token::Paren) {
                let inner;
                parenthesized!(inner in input);
                inner.parse()?
            } else {
                return Err(input.error("expected unit"));
            }
        };

        let neg;
        let exp;

        if input.parse::<Token![^]>().is_ok() {
            neg = if input.parse::<Token![-]>().is_ok() {
                true
            } else if input.parse::<Token![+]>().is_ok() {
                false
            } else {
                false
            };

            exp = Some(input.parse()?);
        } else {
            neg = false;
            exp = None;
        }

        Ok(Self { base, inv, neg, exp })
    }
}

impl<U: UnitCore> UnitExp<U> {
    fn to_unit(self) -> Result<UnitSpec<U>> {
        let unit = match self.exp {
            Some(exp) => {
                let a = exp.numerator();
                let b = exp.denominator();
                let frac: [i32; 2] = [
                    a.base10_parse()?,
                    match b {
                        Some(b) => b.base10_parse()?,
                        None => 1,
                    },
                ];

                let unit = match frac {
                    [_, 0] => return Err(syn::Error::new(
                        b.unwrap().span(),
                        "root of degree zero cannot be defined",
                    )),

                    [0, _] => return Err(syn::Error::new(
                        a.span(),
                        "unit with exponent of zero is scalar",
                    )),

                    [1, 1] => self.base,
                    // [a, b] if a == b => self.base,
                    [_, 1] => UnitSpec::Pow(Box::new(self.base), exp),

                    _ => return Err(syn::Error::new(
                        b.unwrap().span(),
                        "non-whole exponents are currently unsupported",
                        //  TODO
                    )),

                    // _ => UnitSpec::Pow(Box::new(self.base), exp),
                };

                if self.neg {
                    UnitSpec::Inv(Box::new(unit))
                } else {
                    unit
                }
            }
            None => self.base,
        };

        if self.inv {
            Ok(UnitSpec::Inv(Box::new(unit)))
        } else {
            Ok(unit)
        }
    }
}


#[derive(Debug)]
pub enum UnitSpec<U: UnitCore> {
    Leaf(U),

    Div(Box<UnitSpec<U>>, Box<UnitSpec<U>>),
    Mul(Box<UnitSpec<U>>, Box<UnitSpec<U>>),
    Inv(Box<UnitSpec<U>>),
    Pow(Box<UnitSpec<U>>, Exponent),
}

impl<U: UnitCore> Parse for UnitSpec<U> {
    fn parse(input: ParseStream) -> Result<Self> {
        use syn::parse::discouraged::Speculative;

        let mut unit: Self = input.parse::<UnitExp<U>>()?.to_unit()?;

        loop {
            let fork = input.fork();

            unit = if fork.parse::<Token![/]>().is_ok() {
                match fork.parse::<UnitExp<U>>() {
                    Ok(rhs) => Self::Div(unit.into(), rhs.to_unit()?.into()),
                    Err(..) => break,
                }
            } else if fork.parse::<Token![*]>().is_ok() {
                match fork.parse::<UnitExp<U>>() {
                    Ok(rhs) => Self::Mul(unit.into(), rhs.to_unit()?.into()),
                    Err(..) => break,
                }
            } else {
                break;
            };

            input.advance_to(&fork);
        }

        Ok(unit)
    }
}

impl<U: UnitCore> UnitSpec<U> {
    pub fn as_type(&self) -> TokenStream {
        match self {
            Self::Leaf(unit) => unit.to_token_stream(),

            Self::Div(left, right) => {
                let ts_l = left.as_type();
                let ts_r = right.as_type();
                quote!(::dana::units::UnitDiv<#ts_l, #ts_r>)
            }
            Self::Mul(left, right) => {
                let ts_l = left.as_type();
                let ts_r = right.as_type();
                quote!(::dana::units::UnitMul<#ts_l, #ts_r>)
            }
            Self::Inv(unit) => {
                let ts = unit.as_type();
                quote!(::dana::units::PerUnit<#ts>)
            }
            Self::Pow(base, exp) => {
                let ts = base.as_type();
                quote!(::dana::units::UnitPow<#ts, #exp>)
            }
        }
    }

    pub fn as_expr(&self) -> TokenStream {
        match self {
            Self::Leaf(unit) => unit.to_token_stream(),

            Self::Div(left, right) => {
                let ts_l = left.as_expr();
                let ts_r = right.as_expr();
                quote!(::dana::units::UnitDiv(#ts_l, #ts_r))
            }
            Self::Mul(left, right) => {
                let ts_l = left.as_expr();
                let ts_r = right.as_expr();
                quote!(::dana::units::UnitMul(#ts_l, #ts_r))
            }
            Self::Inv(unit) => {
                let ts = unit.as_expr();
                quote!(::dana::units::PerUnit(#ts))
            }
            Self::Pow(base, exp) => {
                let ts = base.as_expr();
                quote!(::dana::units::UnitPow::<_, #exp>::new(#ts))
            }
        }
    }
}

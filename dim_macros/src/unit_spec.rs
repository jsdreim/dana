use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Result,
    Token,
};


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
        leading: bool,
        first: TokenTree,
        path: Vec<syn::Ident>,
        fields: Vec<syn::Ident>,
    },
}

impl Parse for UnitCoreExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let leading = input.parse::<Token![::]>().is_ok();

        if leading || input.peek2(Token![::]) {
            let mut path = Vec::new();
            let mut fields = Vec::new();

            let first = input.parse()?;

            while input.parse::<Token![::]>().is_ok() {
                path.push(input.parse()?);
            }

            while input.parse::<Token![.]>().is_ok() {
                fields.push(input.parse()?);
            }

            Ok(Self::Path { leading, first, path, fields })
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
                first.to_tokens(tokens);

                for ident in fields {
                    quote!(.).to_tokens(tokens);
                    ident.to_tokens(tokens);
                }
            }
            Self::Path { leading, first, path, fields } => {
                if *leading {
                    quote!(::).to_tokens(tokens);
                }

                first.to_tokens(tokens);

                for ident in path {
                    quote!(::).to_tokens(tokens);
                    ident.to_tokens(tokens);
                }

                for ident in fields {
                    quote!(.).to_tokens(tokens);
                    ident.to_tokens(tokens);
                }
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
    Path { leading: bool, first: TokenTree, path: Vec<syn::Ident> },
}

impl Parse for UnitCoreType {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut path = Vec::new();

        let leading = input.parse::<Token![::]>().is_ok();

        if leading || input.peek2(Token![::]) {
            let first = input.parse()?;

            while input.parse::<Token![::]>().is_ok() {
                path.push(input.parse()?);
            }

            Ok(Self::Path { leading, first, path })
        } else {
            Ok(Self::Ident(input.parse()?))
        }
    }
}

impl ToTokens for UnitCoreType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Path { leading, first, path } => {
                if *leading {
                    quote!(::).to_tokens(tokens);
                }

                first.to_tokens(tokens);

                for ident in path {
                    quote!(::).to_tokens(tokens);
                    ident.to_tokens(tokens);
                }
            }
        }
    }
}


#[derive(Debug)]
pub enum Exponent {
    Whole(proc_macro2::Literal),
    Frac(proc_macro2::Literal, proc_macro2::Literal),
}

impl Exponent {
    fn numerator(&self) -> &proc_macro2::Literal {
        match self {
            Self::Whole(a) => a,
            Self::Frac(a, _) => a,
        }
    }

    fn denominator(&self) -> Option<&proc_macro2::Literal> {
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
        let (a, b) = match self {
            Self::Whole(a) => (a, proc_macro2::Literal::i32_unsuffixed(1)),
            Self::Frac(a, b) => (a, b.clone()),
        };

        tokens.extend(quote!(TypeFrac<#a, #b>));
    }
}


#[derive(Debug)]
enum UnitExpBase<U: UnitCore> {
    Base(U),
    Unit(UnitSpec<U>),
}


#[derive(Debug)]
struct UnitExp<U: UnitCore> {
    base: UnitExpBase<U>,
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
            Ok(unit) => UnitExpBase::Base(unit),
            Err(_) => if input.peek(syn::token::Paren) {
                let inner;
                parenthesized!(inner in input);
                UnitExpBase::Unit(inner.parse()?)
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
        let base = match self.base {
            UnitExpBase::Base(base) => UnitSpec::Base(base),
            UnitExpBase::Unit(unit) => unit,
        };

        let unit = match self.exp {
            Some(exp) => {
                let a = exp.numerator();
                let b = exp.denominator();
                let frac: [i32; 2] = [
                    a.to_string().parse().unwrap_or(1),
                    b.and_then(|t| t.to_string().parse().ok()).unwrap_or(1),
                ];
                //  TODO: Correctly check non-decimal literals, like hexadecimal
                //      or binary. Probably need to rebuild around `syn::LitInt`
                //      or similar to achieve this.

                let unit = match frac {
                    [_, 0] => return Err(syn::Error::new(
                        b.unwrap().span(),
                        "root of degree zero cannot be defined",
                    )),

                    [0, _] => return Err(syn::Error::new(
                        a.span(),
                        "unit with exponent of zero is scalar",
                    )),

                    [1, 1] => base,
                    // [a, b] if a == b => base,
                    _ => UnitSpec::Pow(Box::new(base), exp),
                };

                if self.neg {
                    UnitSpec::Inv(Box::new(unit))
                } else {
                    unit
                }
            }
            None => base,
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
    Base(U),

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
            Self::Base(unit) => unit.to_token_stream(),

            Self::Div(left, right) => {
                let ts_l = left.as_type();
                let ts_r = right.as_type();
                quote!(::dimensional::units::UnitDiv<#ts_l, #ts_r>)
            }
            Self::Mul(left, right) => {
                let ts_l = left.as_type();
                let ts_r = right.as_type();
                quote!(::dimensional::units::UnitMul<#ts_l, #ts_r>)
            }
            Self::Inv(unit) => {
                let ts = unit.as_type();
                quote!(::dimensional::units::PerUnit<#ts>)
            }
            Self::Pow(base, exp) => {
                let ts = base.as_type();
                quote!(::dimensional::units::UnitPow<#ts, ::dimensional::units::exp::#exp>)
            }
        }
    }

    pub fn as_expr(&self) -> TokenStream {
        match self {
            Self::Base(unit) => unit.to_token_stream(),

            Self::Div(left, right) => {
                let ts_l = left.as_expr();
                let ts_r = right.as_expr();
                quote!(::dimensional::units::UnitDiv(#ts_l, #ts_r))
            }
            Self::Mul(left, right) => {
                let ts_l = left.as_expr();
                let ts_r = right.as_expr();
                quote!(::dimensional::units::UnitMul(#ts_l, #ts_r))
            }
            Self::Inv(unit) => {
                let ts = unit.as_expr();
                quote!(::dimensional::units::PerUnit(#ts))
            }
            Self::Pow(base, exp) => {
                let ts = base.as_expr();
                quote!(::dimensional::units::UnitPow::<_, ::dimensional::units::exp::#exp>::new(#ts))
            }
        }
    }
}

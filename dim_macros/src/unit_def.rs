use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Result,
    Token,
};


#[derive(Debug)]
pub enum UnitIdent {
    /// One identifier.
    Ident(syn::Ident),
    /// One token, followed by a sequence of field identifiers.
    Field(TokenTree, Vec<syn::Ident>),
    /// One token, followed by a sequence of path identifiers.
    Path(TokenTree, Vec<syn::Ident>),
}

impl Parse for UnitIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut idents = Vec::new();

        if input.peek2(Token![::]) {
            let first = input.parse()?;

            while input.parse::<Token![::]>().is_ok() {
                idents.push(input.parse()?);
            }

            Ok(Self::Path(first, idents))
        } else if input.peek2(Token![.]) {
            let first = input.parse()?;

            while input.parse::<Token![.]>().is_ok() {
                idents.push(input.parse()?);
            }

            Ok(Self::Field(first, idents))
        } else {
            Ok(Self::Ident(input.parse()?))
        }
    }
}

impl ToTokens for UnitIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Field(first, path) => {
                first.to_tokens(tokens);

                for ident in path {
                    quote!(.).to_tokens(tokens);
                    ident.to_tokens(tokens);
                }
            }
            Self::Path(first, path) => {
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
        match input.parse::<proc_macro2::Literal>() {
            Ok(lit) => Ok(Self::Whole(lit)),
            Err(..) => {
                let inner;
                parenthesized!(inner in input);

                let first = inner.parse()?;

                if inner.is_empty() {
                    Ok(Self::Whole(first))
                } else {
                    inner.parse::<Token![/]>()?;
                    Ok(Self::Frac(first, inner.parse()?))
                }
            }
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


type Inner = UnitIdent;


#[derive(Debug)]
enum UnitExpBase<U: Parse + ToTokens = Inner> {
    Base(U),
    Unit(UnitDef<U>),
}


#[derive(Debug)]
struct UnitExp<U: Parse + ToTokens = Inner> {
    base: UnitExpBase<U>,
    inv: bool,
    neg: bool,
    exp: Option<Exponent>,
}

impl<U: Parse + ToTokens> Parse for UnitExp<U> {
    fn parse(input: ParseStream) -> Result<Self> {
        let base;
        let inv;
        let neg;
        let exp;

        if let Ok(numerator) = input.parse::<syn::LitInt>() {
            input.parse::<Token![/]>()?;
            inv = true;

            if numerator.base10_digits() != "1" {
                return Err(syn::Error::new(numerator.span(), "Invalid numerator."));
            }
        } else {
            inv = false;
        }

        if let Ok(unit) = input.parse() {
            base = UnitExpBase::Base(unit);
        } else {
            let inner;
            parenthesized!(inner in input);
            base = UnitExpBase::Unit(inner.parse()?);
        }

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

impl<U: Parse + ToTokens> UnitExp<U> {
    fn to_unit(self) -> Result<UnitDef<U>> {
        let base = match self.base {
            UnitExpBase::Base(base) => UnitDef::Base(base),
            UnitExpBase::Unit(unit) => unit,
        };

        let unit = match self.exp {
            Some(exp) => {
                let a = exp.numerator();
                let b = exp.denominator();
                let a_str = a.to_string();
                let b_str = b.map(|t| t.to_string())
                    .unwrap_or_else(|| String::from("1"));

                let unit = match [a_str.as_str(), b_str.as_str()] {
                    [_, "0"] => return Err(syn::Error::new(
                        b.unwrap().span(),
                        "Root of degree zero cannot be defined.",
                    )),

                    ["0", _] => return Err(syn::Error::new(
                        a.span(),
                        "Unit with exponent of zero is scalar.",
                    )),

                    [a, b] if a == b => base,
                    _ => UnitDef::Pow(Box::new(base), exp),
                };

                if self.neg {
                    UnitDef::Inv(Box::new(unit))
                } else {
                    unit
                }
            }
            None => base,
        };

        if self.inv {
            Ok(UnitDef::Inv(Box::new(unit)))
        } else {
            Ok(unit)
        }
    }
}


#[derive(Debug)]
enum Operation<U: Parse + ToTokens> {
    Div(UnitExp<U>),
    Mul(UnitExp<U>),
}


#[derive(Debug)]
pub enum UnitDef<U: Parse + ToTokens = Inner> {
    Base(U),

    Div(Box<UnitDef<U>>, Box<UnitDef<U>>),
    Mul(Box<UnitDef<U>>, Box<UnitDef<U>>),
    Inv(Box<UnitDef<U>>),
    Pow(Box<UnitDef<U>>, Exponent),
}

impl<U: Parse + ToTokens> Parse for UnitDef<U> {
    fn parse(input: ParseStream) -> Result<Self> {
        let left: Result<UnitExp<U>> = input.parse();
        let mut ops = Vec::new();

        loop {
            if input.parse::<Token![/]>().is_ok() {
                ops.push(input.parse().map(Operation::Div));
                continue;
            } else if input.parse::<Token![*]>().is_ok() {
                ops.push(input.parse().map(Operation::Mul));
                continue;
            } else {
                break;
            }

            // unreachable!()
        }

        // return Err(input.error("asdf"));

        let mut out = left?.to_unit()?;

        for op in ops {
            out = match op? {
                Operation::Div(right) => {
                    Self::Div(Box::new(out), Box::new(right.to_unit()?))
                }
                Operation::Mul(right) => {
                    Self::Mul(Box::new(out), Box::new(right.to_unit()?))
                }
            };
        }

        Ok(out)
    }
}

impl<U: Parse + ToTokens> UnitDef<U> {
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

    pub fn as_value(&self) -> TokenStream {
        match self {
            Self::Base(unit) => unit.to_token_stream(),

            Self::Div(left, right) => {
                let ts_l = left.as_value();
                let ts_r = right.as_value();
                quote!(::dimensional::units::UnitDiv(#ts_l, #ts_r))
            }
            Self::Mul(left, right) => {
                let ts_l = left.as_value();
                let ts_r = right.as_value();
                quote!(::dimensional::units::UnitMul(#ts_l, #ts_r))
            }
            Self::Inv(unit) => {
                let ts = unit.as_value();
                quote!(::dimensional::units::PerUnit(#ts))
            }
            Self::Pow(base, exp) => {
                let ts = base.as_value();
                quote!(::dimensional::units::UnitPow::<_, ::dimensional::units::exp::#exp>::new(#ts))
            }
        }
    }
}

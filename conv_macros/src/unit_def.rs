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


type Inner = UnitIdent;


#[derive(Debug)]
enum UnitExpBase<U: Parse + ToTokens = Inner> {
    Base(U),
    Recurse(UnitDef<U>),
}


#[derive(Debug)]
struct UnitExp<U: Parse + ToTokens = Inner> {
    base: UnitExpBase<U>,
    inv: bool,
    neg: bool,
    exp: Option<proc_macro2::Literal>,
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
            base = UnitExpBase::Recurse(inner.parse()?);
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
        let base: UnitDef<U> = match self.base {
            UnitExpBase::Base(base) => UnitDef::Base(base),
            UnitExpBase::Recurse(unit) => unit,
        };

        let unit = match self.exp {
            None => base,
            Some(exp) => match (self.neg, exp.to_string().as_str()) {
                (true,  "3") => UnitDef::Inv(Box::new(UnitDef::Pow3(Box::new(base)))),
                (true,  "2") => UnitDef::Inv(Box::new(UnitDef::Pow2(Box::new(base)))),
                (true,  "1") => UnitDef::Inv(Box::new(base)),
                (false, "0") => todo!("zero exponent"),
                (false, "1") => base,
                (false, "2") => UnitDef::Pow2(Box::new(base)),
                (false, "3") => UnitDef::Pow3(Box::new(base)),

                _ => {
                    return Err(syn::Error::new(exp.span(), "Invalid exponent"));
                }
            }
        };

        if self.inv {
            // match unit {
            //     UnitDef::Inv(unit) => Ok(*unit),
            //     unit => Ok(UnitDef::Inv(Box::new(unit))),
            // }
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
    Pow2(Box<UnitDef<U>>),
    Pow3(Box<UnitDef<U>>),
    // Pow4(Box<UnitDef<U>>),
    // Pow(Box<UnitDef<U>>, i32),
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
                quote!(UnitDiv<#ts_l, #ts_r>)
            }
            Self::Mul(left, right) => {
                let ts_l = left.as_type();
                let ts_r = right.as_type();
                quote!(UnitMul<#ts_l, #ts_r>)
            }
            Self::Inv(unit) => {
                let ts = unit.as_type();
                quote!(PerUnit<#ts>)
            }
            Self::Pow2(base) => {
                let ts = base.as_type();
                quote!(UnitSquared<#ts>)
            }
            Self::Pow3(base) => {
                let ts = base.as_type();
                quote!(UnitCubed<#ts>)
            }
        }
    }

    pub fn as_value(&self) -> TokenStream {
        match self {
            Self::Base(unit) => unit.to_token_stream(),

            Self::Div(left, right) => {
                let ts_l = left.as_value();
                let ts_r = right.as_value();
                quote!(UnitDiv(#ts_l, #ts_r))
            }
            Self::Mul(left, right) => {
                let ts_l = left.as_value();
                let ts_r = right.as_value();
                quote!(UnitMul(#ts_l, #ts_r))
            }
            Self::Inv(unit) => {
                let ts = unit.as_value();
                quote!(PerUnit(#ts))
            }
            Self::Pow2(base) => {
                let ts = base.as_value();
                quote!(UnitSquared(#ts))
            }
            Self::Pow3(base) => {
                let ts = base.as_value();
                quote!(UnitCubed(#ts))
            }
        }
    }
}

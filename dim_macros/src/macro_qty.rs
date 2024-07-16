use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    Result,
    Token,
};
use crate::unit_def::*;


#[derive(Debug)]
pub struct QtyNew {
    pub value: proc_macro2::Literal,
    pub unit: UnitDef,
}

impl Parse for QtyNew {
    fn parse(input: ParseStream) -> Result<Self> {
        use syn::parse::discouraged::Speculative;

        let fork = input.fork();
        let value = fork.parse()?;

        let unit: UnitDef = if fork.parse::<Token![/]>().is_ok() {
            UnitDef::Inv(Box::new(fork.parse()?))
        } else if fork.parse::<Token![*]>().is_ok() {
            fork.parse()?
        } else {
            fork.parse()?
        };

        input.advance_to(&fork);

        Ok(Self { value, unit })
    }
}


#[derive(Debug)]
pub enum QtyBase {
    New(QtyNew),
    Pass(TokenTree),
}

impl Parse for QtyBase {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(value) = input.parse() {
            //  Definition of a new quantity.

            let unit: UnitDef = if input.parse::<Token![/]>().is_ok() {
                UnitDef::Inv(Box::new(input.parse()?))
            } else if input.parse::<Token![*]>().is_ok() {
                input.parse()?
            } else {
                input.parse()?
            };

            Ok(Self::New(QtyNew { value, unit }))
        } else {
            //  Usage of an existing quantity.
            Ok(Self::Pass(input.parse()?))
        }
    }
}


struct Recursion(MacroQty);

impl Parse for Recursion {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner;
        bracketed!(inner in input);
        Ok(Self(inner.parse()?))
    }
}


pub enum Op {
    Convert,
    ConvertType(UnitDef),
    ConvertUnit(UnitDef),
    Simplify,
    SimplifyType(UnitDef),
    Binary {
        op: TokenTree,
        rhs: TokenTree,
    },
}

impl Parse for Op {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.parse::<Token![as]>().is_ok() {
            if input.parse::<Token![_]>().is_ok() {
                Ok(Self::Convert)
            } else {
                Ok(Self::ConvertType(input.parse()?))
            }
        } else if input.parse::<Token![in]>().is_ok() {
            if input.parse::<Token![_]>().is_ok() {
                Ok(Self::Convert)
            } else {
                Ok(Self::ConvertUnit(input.parse()?))
            }
        } else if input.parse::<Token![->]>().is_ok() {
            if input.parse::<Token![_]>().is_ok() {
                Ok(Self::Simplify)
            } else {
                Ok(Self::SimplifyType(input.parse()?))
            }
        } else {
            Ok(Self::Binary {
                op: input.parse()?,
                rhs: input.parse()?,
            })
        }
    }
}


#[derive(Debug)]
pub enum MacroQty {
    /// Define a new quantity.
    New(QtyNew),
    /// Retrieve the scalar value of a quantity.
    Deref(Box<Self>),

    /// Convert a quantity to the default of an inferred unit type.
    Convert {
        qty: Box<Self>,
    },
    /// Convert a quantity to the default of a specified unit type.
    ConvertType {
        qty: Box<Self>,
        utype: UnitDef,
    },
    /// Convert a quantity to a specified unit.
    ConvertUnit {
        qty: Box<Self>,
        unit: UnitDef,
    },

    /// Simplify a quantity to an inferred unit type.
    Simplify {
        qty: Box<Self>,
    },
    /// Simplify a quantity to a specified unit type.
    SimplifyType {
        qty: Box<Self>,
        utype: UnitDef,
    },

    /// Perform a binary operation between quantities.
    Operation {
        op: TokenTree,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    /// Passthrough.
    Pass(TokenStream),
}

impl MacroQty {
    fn deref(self) -> Self {
        Self::Deref(self.into())
    }

    fn convert(self) -> Self {
        Self::Convert { qty: self.into() }
    }

    fn convert_type(self, utype: UnitDef) -> Self {
        Self::ConvertType { qty: self.into(), utype }
    }

    fn convert_unit(self, unit: UnitDef) -> Self {
        Self::ConvertUnit { qty: self.into(), unit }
    }

    fn simplify(self) -> Self {
        Self::Simplify { qty: self.into() }
    }

    fn simplify_type(self, utype: UnitDef) -> Self {
        Self::SimplifyType { qty: self.into(), utype }
    }

    fn op(self, op: TokenTree, rhs: Self) -> Self {
        Self::Operation { op, lhs: self.into(), rhs: rhs.into() }
    }
}

impl Parse for MacroQty {
    fn parse(input: ParseStream) -> Result<Self> {
        let deref: bool = input.parse::<Token![*]>().is_ok();
        let base: QtyBase = input.parse()?;
        let mut ops: Vec<Op> = Vec::new();

        while let Ok(op) = input.parse() {
            ops.push(op);
        }

        let mut qty: Self = match base {
            QtyBase::New(new) => Self::New(new),
            QtyBase::Pass(tt) => match syn::parse2(tt.to_token_stream()) {
                Ok(Recursion(qty)) => qty,
                Err(_) => Self::Pass(tt.into_token_stream()),
            }
        };

        for op in ops {
            qty = match op {
                Op::Convert             => qty.convert(),
                Op::ConvertType(utype)  => qty.convert_type(utype),
                Op::ConvertUnit(unit)   => qty.convert_unit(unit),
                Op::Simplify            => qty.simplify(),
                Op::SimplifyType(utype) => qty.simplify_type(utype),
                Op::Binary { op, rhs }  => qty.op(op, syn::parse2(rhs.into_token_stream())?),
            }
        }

        if deref {
            Ok(qty.deref())
        } else {
            Ok(qty)
        }
    }
}

impl ToTokens for MacroQty {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::New(QtyNew { value, unit }) => {
                let unit = unit.as_value();

                tokens.extend(quote! {
                    ::dimensional::Quantity {
                        value: #value,
                        unit: #unit,
                    }
                });
            }
            Self::Deref(qty) => {
                tokens.extend(quote!((#qty.value)));
            }
            Self::Convert { qty } => {
                tokens.extend(quote!(#qty.convert()));
            }
            Self::ConvertType { qty, utype } => {
                let utype = utype.as_type();
                tokens.extend(quote!(#qty.convert::<#utype>()));
            }
            Self::ConvertUnit { qty, unit } => {
                let unit = unit.as_value();
                tokens.extend(quote!(#qty.convert_to(#unit)));
            }
            Self::Simplify { qty } => {
                tokens.extend(quote!(#qty.simplify()));
            }
            Self::SimplifyType { qty, utype } => {
                let utype = utype.as_type();
                // eprintln!("{utype}");
                tokens.extend(quote!(#qty.simplify::<#utype>()));
            }
            Self::Operation { op, lhs, rhs } => {
                tokens.extend(quote!((#lhs #op #rhs)));
            }
            Self::Pass(ts) => ts.to_tokens(tokens),
        }
    }
}

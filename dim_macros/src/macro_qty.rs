use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    Result,
    Token,
};
use crate::unit_def::*;


pub struct QtyNew {
    pub sign: Option<syn::token::Minus>,
    pub value: proc_macro2::Literal,
    pub unit: UnitDef,
}

impl std::fmt::Debug for QtyNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "QtyNew {{ sign: {}, value: {:?}, unit: {:?} }}",
            match self.sign {
                Some(m) => format!("Some(Minus {{ spans: {:?} }})", m.spans),
                None => String::from("None"),
            },
            self.value,
            self.unit,
        )
    }
}

impl Parse for QtyNew {
    fn parse(input: ParseStream) -> Result<Self> {
        let sign = input.parse::<Token![-]>().ok();
        let value = input.parse()?;

        let unit: UnitDef = if input.parse::<Token![/]>().is_ok() {
            UnitDef::Inv(Box::new(input.parse()?))
        } else if input.parse::<Token![*]>().is_ok() {
            input.parse()?
        } else {
            input.parse()?
        };

        Ok(Self { sign, value, unit })
    }
}

impl ToTokens for QtyNew {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let sign = self.sign;
        let value = &self.value;
        let unit = self.unit.as_value();

        tokens.extend(quote!(::dimensional::Quantity {
            value: #sign #value,
            unit: #unit,
        }));
    }
}


#[derive(Debug)]
pub enum QtyBase {
    New(QtyNew, Vec<QtyNew>),
    Pass(TokenTree),
}

impl Parse for QtyBase {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.fork().parse::<QtyNew>().is_ok() {
            //  At least one new quantity definition.
            let mut add = Vec::new();

            //  Read new quantities.
            while let Ok(new) = input.parse() {
                add.push(new);

                if input.parse::<Token![,]>().is_ok() {
                    //  Comma separator.
                } else if input.parse::<Token![+]>().is_ok() {
                    //  Plus separator.
                }
            }

            add.rotate_left(1);
            let new = add.pop()
                .expect("qty found in fork of stream, but not in real stream");

            Ok(Self::New(new, add))
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
    New(QtyNew, Vec<QtyNew>),
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
            QtyBase::New(new, add) => Self::New(new, add),
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
            // Self::New(new, _) => new.to_tokens(tokens),
            Self::New(new, add) if add.is_empty() => new.to_tokens(tokens),
            Self::New(new, add) => {
                tokens.extend(quote!((#new #(+ #add)*)));
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

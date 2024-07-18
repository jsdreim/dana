use proc_macro2::{Group, Literal, Punct, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    bracketed,
    parse::{discouraged::Speculative, Parse, ParseStream},
    Result,
    Token,
};
use crate::unit_def::*;


pub struct QtyNew {
    pub sign: Option<syn::token::Minus>,
    pub value: Literal,
    pub unit: UnitSpec,
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
        let fork = input.fork();

        //  Check for a negative sign.
        let sign = fork.parse::<Token![-]>().ok();

        //  Read a scalar value.
        let value = fork.parse()?;

        //  Check for an optional div/mul operator.
        let inv = if fork.parse::<Token![/]>().is_ok() {
            //  `1.0 / s`
            true
        } else if fork.parse::<Token![*]>().is_ok() {
            //  `1.0 * s`
            false
        } else {
            //  `1.0 s`
            false
        };

        //  Read a unit specifier, possibly inverting it.
        let unit = if inv {
            UnitSpec::Inv(Box::new(fork.parse()?))
        } else {
            fork.parse()?
        };

        input.advance_to(&fork);

        Ok(Self { sign, value, unit })
    }
}

impl ToTokens for QtyNew {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let sign = self.sign;
        let value = &self.value;
        let unit = self.unit.as_expr();

        tokens.extend(quote!(::dimensional::Quantity {
            value: #sign #value,
            unit: #unit,
        }));
    }
}


#[derive(Debug)]
pub enum QtyBase {
    New(QtyNew, Vec<QtyNew>),
    Recursive(MacroQty<false>),
    PassIdent(syn::Ident),
    PassGroup(Group),
}

impl Parse for QtyBase {
    fn parse(input: ParseStream) -> Result<Self> {
        match input.parse::<QtyNew>() {
            Ok(first) => {
                //  At least one new quantity definition.
                let mut add = Vec::new();

                loop {
                    //  Allow a comma here.
                    input.parse::<Token![,]>().ok();

                    //  Try to parse another quantity.
                    match input.parse::<QtyNew>() {
                        Ok(new) => add.push(new),
                        Err(_) => break,
                    }
                }

                Ok(Self::New(first, add))
            }
            Err(e) => if input.peek(syn::token::Bracket) {
                let inner;
                bracketed!(inner in input);
                Ok(Self::Recursive(inner.parse()?))
            } else if let Ok(ident) = input.parse() {
                Ok(Self::PassIdent(ident))
            } else if let Ok(group) = input.parse() {
                Ok(Self::PassGroup(group))
            } else {
                Err(e)
            }
        }
    }
}


#[derive(Debug)]
pub enum Op {
    Convert,
    ConvertType(UnitSpec),
    ConvertUnit(UnitSpec),
    Simplify,
    SimplifyType(UnitSpec),
    Binary {
        op: Punct,
        rhs: MacroQty<false>,
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
            let fork = input.fork();
            let op = Self::Binary {
                op: fork.parse()?,
                rhs: fork.parse::<QtyBase>()?.into(),
            };

            input.advance_to(&fork);
            Ok(op)
        }
    }
}


#[derive(Debug)]
pub enum MacroQty<const TOP: bool = true> {
    /// Define a new quantity.
    New(QtyNew, Vec<QtyNew>),
    /// Retrieve the scalar value of a quantity.
    Deref(Box<MacroQty<false>>),

    /// Convert a quantity to the default of an inferred unit type.
    Convert {
        qty: Box<MacroQty<false>>,
    },
    /// Convert a quantity to the default of a specified unit type.
    ConvertType {
        qty: Box<MacroQty<false>>,
        utype: UnitSpec,
    },
    /// Convert a quantity to a specified unit.
    ConvertUnit {
        qty: Box<MacroQty<false>>,
        unit: UnitSpec,
    },

    /// Simplify a quantity to an inferred unit type.
    Simplify {
        qty: Box<MacroQty<false>>,
    },
    /// Simplify a quantity to a specified unit type.
    SimplifyType {
        qty: Box<MacroQty<false>>,
        utype: UnitSpec,
    },

    /// Perform a binary operation between quantities.
    Binary {
        lhs: Box<MacroQty<false>>,
        op: Punct,
        rhs: Box<MacroQty<false>>,
    },
    /// Passthrough.
    Pass(TokenStream),
}

impl<const A: bool> MacroQty<A> {
    fn apply_operation(self, op: Op) -> Self {
        match op {
            Op::Convert
            => Self::Convert { qty: self.demote() },

            Op::ConvertType(utype)
            => Self::ConvertType { qty: self.demote(), utype },

            Op::ConvertUnit(unit)
            => Self::ConvertUnit { qty: self.demote(), unit },

            Op::Simplify
            => Self::Simplify { qty: self.demote() },

            Op::SimplifyType(utype)
            => Self::SimplifyType { qty: self.demote(), utype },

            Op::Binary { op, rhs }
            => Self::Binary { lhs: self.demote(), op, rhs: rhs.into() },
        }
    }

    fn to_level<const B: bool>(self) -> MacroQty<B> {
        unsafe { std::mem::transmute(self) }
    }

    fn demote(self) -> Box<MacroQty<false>> {
        Box::new(self.to_level())
    }

    fn deref(self) -> Self {
        Self::Deref(self.demote())
    }
}

impl<const TOP: bool> From<QtyBase> for MacroQty<TOP> {
    fn from(base: QtyBase) -> Self {
        match base {
            QtyBase::New(new, add) => Self::New(new, add),
            QtyBase::Recursive(inner) => inner.to_level(),
            QtyBase::PassIdent(ident) => Self::Pass(ident.into_token_stream()),
            QtyBase::PassGroup(group) => Self::Pass(group.into_token_stream()),
        }
    }
}

impl<const TOP: bool> Parse for MacroQty<TOP> {
    fn parse(input: ParseStream) -> Result<Self> {
        //  If this is the top level, check for a "deref" sigil.
        let deref: bool = TOP && input.parse::<Token![*]>().is_ok();

        //  Parse a single item. Either a new quantity literal or something to
        //      pass through unchanged.
        let mut qty: Self = input.parse::<QtyBase>()?.into();

        //  Read and apply as many transformations and operations as are found.
        while let Ok(op) = input.parse() {
            qty = qty.apply_operation(op);
        }

        //  Dereference, if needed.
        if deref {
            qty = qty.deref();
        }

        Ok(qty)
    }
}

impl<const TOP: bool> ToTokens for MacroQty<TOP> {
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
                let unit = unit.as_expr();
                tokens.extend(quote!(#qty.convert_to(#unit)));
            }
            Self::Simplify { qty } => {
                tokens.extend(quote!(#qty.simplify()));
            }
            Self::SimplifyType { qty, utype } => {
                let utype = utype.as_type();
                tokens.extend(quote!(#qty.simplify::<#utype>()));
            }
            Self::Binary { lhs, op, rhs } => {
                tokens.extend(quote!((#lhs #op #rhs)));
            }
            Self::Pass(ts) => ts.to_tokens(tokens),
        }
    }
}

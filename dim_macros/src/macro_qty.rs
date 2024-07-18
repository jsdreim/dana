use proc_macro2::{Group, Literal, Punct, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    bracketed,
    parse::{discouraged::Speculative, Parse, ParseStream},
    Result,
    Token,
};
use crate::unit_def::*;


#[derive(Debug)]
pub enum QtyValue {
    Literal(Literal),
    Ident(syn::Ident),
    Group(Group),
}

impl QtyValue {
    pub const fn is_literal(&self) -> bool { matches!(self, Self::Literal(_)) }
}

impl Parse for QtyValue {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(lit) = input.parse() {
            Ok(Self::Literal(lit))
        } else if let Ok(ident) = input.parse() {
            Ok(Self::Ident(ident))
        } else if let Ok(group) = input.parse() {
            Ok(Self::Group(group))
        } else {
            Err(input.error("expected literal, identifier, or group"))
        }
    }
}

impl ToTokens for QtyValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Literal(lit) => lit.to_tokens(tokens),
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Group(group) => group.to_tokens(tokens),
        }
    }
}


#[derive(Debug)]
pub struct QtyNew {
    pub sign: Option<crate::util::Sign>,
    pub value: QtyValue,
    pub unit: UnitSpec,
}

impl Parse for QtyNew {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();

        //  Check for a negative sign.
        let sign = fork.parse().ok();

        //  Read a scalar value.
        let value: QtyValue = fork.parse()?;

        //  Check for an optional div/mul operator.
        let inv = value.is_literal() && if fork.parse::<Token![/]>().is_ok() {
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
pub enum SingleQty {
    New(QtyNew, Vec<QtyNew>),
    Recursive(MacroQty<false>),
    PassIdent(syn::Ident),
    PassGroup(Group),
}

impl SingleQty {
    fn promote(self) -> Box<MacroQty<false>> { Box::new(self.into()) }
}

impl Parse for SingleQty {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Err(input.error("expected quantity"));
        }

        match input.parse::<QtyNew>() {
            Ok(first) => {
                //  At least one new quantity definition.
                let mut add = Vec::new();

                loop {
                    //  Should we *expect* to find another quantity?
                    let expecting_another = {
                        if input.parse::<Token![+]>().is_ok() {
                            //  Found a plus sign, should definitely be another
                            //      quantity after this.
                            true
                        } else if input.parse::<Token![,]>().is_ok() {
                            //  Found a comma. Could be another, but this is
                            //      allowed to be trailing.
                            false
                        } else {
                            //  Found no indication. There may or may not be
                            //      another quantity.
                            false
                        } /*else {
                            //  Found no indication. Specifically expect NOT to
                            //      find another quantity.
                            break;
                        }*/
                    };

                    //  Try to parse another quantity.
                    match input.parse::<QtyNew>() {
                        Ok(new) => add.push(new),
                        Err(_) => if expecting_another {
                            return Err(input.error("expected another quantity"));
                        } else {
                            break;
                        }
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
        rhs: SingleQty,
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

            let Ok(op) = fork.parse::<Punct>() else {
                return Err(fork.error("expected `as`, `in`, `->`, or operator"));
            };

            let Ok(rhs) = fork.parse::<SingleQty>() else {
                return Err(fork.error("expected another quantity"));
            };

            input.advance_to(&fork);
            Ok(Self::Binary { op, rhs })
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
            => Self::Binary {
                lhs: self.demote(),
                op,
                rhs: rhs.promote(),
            },
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

impl<const TOP: bool> From<SingleQty> for MacroQty<TOP> {
    fn from(base: SingleQty) -> Self {
        match base {
            SingleQty::New(new, add) => Self::New(new, add),
            SingleQty::Recursive(inner) => inner.to_level(),
            SingleQty::PassIdent(ident) => Self::Pass(ident.into_token_stream()),
            SingleQty::PassGroup(group) => Self::Pass(group.into_token_stream()),
        }
    }
}

impl<const TOP: bool> Parse for MacroQty<TOP> {
    fn parse(input: ParseStream) -> Result<Self> {
        //  If this is the top level, check for a "deref" sigil.
        let deref: bool = TOP && input.parse::<Token![*]>().is_ok();

        //  Parse a single item. Either a new quantity literal or something to
        //      pass through unchanged.
        let mut qty: Self = input.parse::<SingleQty>()?.into();

        //  Read and apply as many transformations and operations as are found.
        while !input.is_empty() {
            qty = qty.apply_operation(input.parse()?);
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

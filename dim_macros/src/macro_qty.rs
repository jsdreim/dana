use proc_macro2::{Group, Literal, Punct, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse2,
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
        use syn::parse::discouraged::Speculative;

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

                    //  Try to parse a quantity.
                    match input.parse::<QtyNew>() {
                        Ok(new) => {
                            //  Parsed another quantity. Put it into the list of
                            //      quantities to add to the initial one.
                            add.push(new);
                            continue;
                        }
                        Err(e) if expecting_another => {
                            //  Could not parse another quantity, but expected
                            //      to, due to parsing a conjunction character.
                            //      Return the parse error.
                            return Err(e);
                        }
                        Err(_) => {
                            //  Could not parse another quantity, but did not
                            //      expect to find one. Simply stop trying.
                            break;
                        }
                    }

                    // unreachable!();
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


pub enum Op {
    Convert,
    ConvertType(UnitSpec),
    ConvertUnit(UnitSpec),
    Simplify,
    SimplifyType(UnitSpec),
    Binary {
        op: Punct,
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
    Operation {
        op: Punct,
        lhs: Box<MacroQty<false>>,
        rhs: Box<MacroQty<false>>,
    },
    /// Passthrough.
    Pass(TokenStream),
}

impl<const A: bool> MacroQty<A> {
    fn to_level<const B: bool>(self) -> MacroQty<B> {
        unsafe { std::mem::transmute(self) }
    }

    fn demote(self) -> Box<MacroQty<false>> {
        Box::new(self.to_level())
    }

    fn deref(self) -> Self {
        Self::Deref(self.demote())
    }

    fn convert(self) -> Self {
        Self::Convert { qty: self.demote() }
    }

    fn convert_type(self, utype: UnitSpec) -> Self {
        Self::ConvertType { qty: self.demote(), utype }
    }

    fn convert_unit(self, unit: UnitSpec) -> Self {
        Self::ConvertUnit { qty: self.demote(), unit }
    }

    fn simplify(self) -> Self {
        Self::Simplify { qty: self.demote() }
    }

    fn simplify_type(self, utype: UnitSpec) -> Self {
        Self::SimplifyType { qty: self.demote(), utype }
    }

    fn op(self, op: Punct, rhs: Self) -> Self {
        Self::Operation { op, lhs: self.demote(), rhs: rhs.demote() }
    }
}

impl<const TOP: bool> Parse for MacroQty<TOP> {
    fn parse(input: ParseStream) -> Result<Self> {
        //  If this is the top level, check for a "deref" sigil.
        let deref: bool = TOP && input.parse::<Token![*]>().is_ok();

        //  Parse a single item. Either a new quantity literal or something to
        //      pass through unchanged.
        let mut qty: Self = match input.parse()? {
            QtyBase::New(new, add) => Self::New(new, add),
            QtyBase::Recursive(inner) => inner.to_level(),
            QtyBase::PassIdent(ident) => Self::Pass(ident.into_token_stream()),
            QtyBase::PassGroup(group) => Self::Pass(group.into_token_stream()),
        };

        //  Read and apply as many transformations and operations as are found.
        while let Ok(op) = input.parse() {
            qty = match op {
                Op::Convert             => qty.convert(),
                Op::ConvertType(utype)  => qty.convert_type(utype),
                Op::ConvertUnit(unit)   => qty.convert_unit(unit),
                Op::Simplify            => qty.simplify(),
                Op::SimplifyType(utype) => qty.simplify_type(utype),
                Op::Binary { op, rhs }  => qty.op(op, parse2(rhs.into_token_stream())?),
            }
        }

        if deref {
            Ok(qty.deref())
        } else {
            Ok(qty)
        }
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

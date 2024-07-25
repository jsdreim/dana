use proc_macro2::{Group, Literal, Punct, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    bracketed,
    parse::{discouraged::Speculative, Parse, ParseStream},
    Result,
    Token,
};
use crate::unit_spec::{UnitSpecExpr, UnitSpecType};


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
    pub unit: UnitSpecExpr,
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
            UnitSpecExpr::Inv(Box::new(fork.parse()?))
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
pub enum QtySingle {
    /// Define a new quantity.
    New(QtyNew, Vec<QtyNew>),
    /// Call the macro recursively.
    Recursive(Box<QtyTree>),
    /// Pass one ident through unaffected.
    PassIdent(syn::Ident),
    /// Pass one group through unaffected.
    PassGroup(Group),
}

impl Parse for QtySingle {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Err(input.error("expected quantity"));
        }

        match input.parse::<QtyNew>() {
            Ok(first) => {
                //  At least one new quantity definition.
                let mut add = Vec::new();

                while input.parse::<Token![,]>().is_ok() {
                    /*//  TODO: Disallow trailing comma?
                    if input.is_empty() {
                        return Err(input.error("expected quantity"));
                    }

                    match input.parse::<QtyNew>() {
                        Ok(new) => add.push(new),
                        Err(..) => return Err(input.error("expected quantity")),
                        // Err(e) => return Err(e),
                    }*/

                    //  Try to parse another quantity.
                    match input.parse::<QtyNew>() {
                        Ok(new) => add.push(new),
                        Err(..) => break,
                    }
                }

                Ok(Self::New(first, add))
            }
            Err(e) => if input.peek(syn::token::Bracket) {
                let inner;
                bracketed!(inner in input);
                Ok(Self::Recursive(Box::new(inner.parse()?)))
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

impl ToTokens for QtySingle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::New(first, add) if !add.is_empty() => {
                tokens.extend(quote!((#first #(+ #add)*)));
            }
            Self::New(first, _add) => first.to_tokens(tokens),
            Self::Recursive(inner) => inner.to_tokens(tokens),
            Self::PassIdent(ident) => ident.to_tokens(tokens),
            Self::PassGroup(group) => group.to_tokens(tokens),
        }
    }
}


#[derive(Debug)]
pub struct QtyNode {
    pub qty: QtySingle,
    pub fields: Vec<syn::Ident>,
}

impl Parse for QtyNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let qty = input.parse()?;
        let mut fields = Vec::new();

        // if let Ok(dot) = input.parse::<Token![.]>() {
        //     let e_span = match input.parse::<syn::Ident>() {
        //         Ok(id) => dot.span.join(id.span()).unwrap_or(dot.span),
        //         Err(_) => dot.span,
        //     };
        //
        //     return Err(syn::Error::new(e_span, "field access forbidden"));
        // }

        while input.parse::<Token![.]>().is_ok() {
            fields.push(input.parse()?);
        }

        Ok(Self { qty, fields })
    }
}

impl ToTokens for QtyNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { qty, fields } = self;
        tokens.extend(quote!(#qty #(.#fields)*))
    }
}


#[derive(Debug)]
pub enum Operation {
    /// Switch a quantity to an anonymous unit.
    Anonymous,
    /// Convert a quantity to the default of an inferred unit type.
    Convert,
    /// Convert a quantity to the default of a specified unit type.
    ConvertType(UnitSpecType),
    /// Convert a quantity to a specified unit.
    ConvertUnit(UnitSpecExpr),

    /// Simplify a quantity to an inferred unit type.
    Simplify,
    /// Simplify a quantity to a specified unit type.
    SimplifyType(UnitSpecType),

    /// Perform a binary operation between quantities.
    Binary {
        /// Binary operator character.
        op: Punct,
        /// Other quantity involved in the operation.
        rhs: QtyNode,
    },
}

impl Parse for Operation {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.parse::<Token![as]>().is_ok() {
            if input.parse::<Token![?]>().is_ok() {
                Ok(Self::Anonymous)
            } else if input.parse::<Token![_]>().is_ok() {
                Ok(Self::Convert)
            } else {
                Ok(Self::ConvertType(input.parse()?))
            }
        } else if input.parse::<Token![in]>().is_ok() {
            if input.parse::<Token![?]>().is_ok() {
                Ok(Self::Anonymous)
            } else if input.parse::<Token![_]>().is_ok() {
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

            let rhs = fork.parse()?;

            input.advance_to(&fork);
            Ok(Self::Binary { op, rhs })
        }
    }
}


#[derive(Debug)]
pub enum QtyTree {
    /// A single quantity.
    Leaf(QtyNode),

    /// A quantity with some kind of operation to perform.
    Branch(Box<Self>, Operation),
}

impl Parse for QtyTree {
    fn parse(input: ParseStream) -> Result<Self> {
        //  Parse a single item. Either a new quantity literal or something to
        //      pass through unchanged.
        let mut qty: Self = Self::Leaf(input.parse()?);

        //  Read and apply as many transformations and operations as are found.
        while !input.is_empty() {
            qty = Self::Branch(Box::new(qty), input.parse()?);
        }

        Ok(qty)
    }
}

impl ToTokens for QtyTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Leaf(qty) => qty.to_tokens(tokens),
            Self::Branch(qty, op) => tokens.extend(match op {
                Operation::Anonymous => {
                    quote!(#qty.with_anonymous())
                }
                Operation::Convert => {
                    quote!(#qty.convert())
                }
                Operation::ConvertType(utype) => {
                    let utype = utype.as_type();
                    quote!(#qty.convert::<#utype>())
                }
                Operation::ConvertUnit(unit) => {
                    let unit = unit.as_expr();
                    quote!(#qty.convert_to(#unit))
                }
                Operation::Simplify => {
                    quote!(#qty.convert())
                    // quote!(#qty.simplify()) // TODO
                }
                Operation::SimplifyType(utype) => {
                    let utype = utype.as_type();
                    quote!(#qty.convert::<#utype>())
                    // quote!(#qty.simplify::<#utype>()) // TODO
                }
                Operation::Binary { op, rhs } => {
                    quote!((#qty #op #rhs))
                }
            }),
        }
    }
}


#[derive(Debug)]
pub struct MacroQty {
    pub deref: bool,
    pub tree: QtyTree,
}

impl Parse for MacroQty {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            deref: input.parse::<Token![*]>().is_ok(),
            tree: input.parse::<QtyTree>()?,
        })
    }
}

impl ToTokens for MacroQty {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.tree.to_tokens(tokens);

        if self.deref {
            tokens.extend(quote!(.value));
        }
    }
}

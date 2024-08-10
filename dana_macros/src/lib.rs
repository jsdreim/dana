#[macro_use]
mod debug;
#[macro_use]
mod util;

mod macro_dim;
mod macro_qty;
mod macro_qtype;
mod macro_scale;
mod unit_spec;

use proc_macro::TokenStream;
use quote::ToTokens;
use macro_dim::MacroDim;
use macro_qty::MacroQty;
use macro_qtype::MacroQType;
use macro_scale::MacroScale;
use unit_spec::{UnitSpecExpr, UnitSpecType};


#[proc_macro]
pub fn impl_typenums(_: TokenStream) -> TokenStream {
    let mut out = proc_macro2::TokenStream::new();

    let lim: i32 = 64;

    for i in -lim..=lim {
        let ident = syn::Ident::new(
            &util::typenum_int(i),
            proc_macro2::Span::call_site(),
        );

        out.extend(quote::quote! {
            impl HasTypenum for Exponent<#i> {
                type Typenum = ::typenum::consts::#ident;
            }
        });
    }

    out.into()
}


#[proc_macro]
pub fn impl_scale(stream: TokenStream) -> TokenStream {
    let code = macro_dbg! {
        // as macro "impl_scale" for stream;
        syn::parse_macro_input!(stream as MacroScale)
    };
    code.into_token_stream().into()
}


#[proc_macro]
pub fn dim(stream: TokenStream) -> TokenStream {
    wrap_dbg!(MacroDim as MacroDimTop);

    let dim = macro_dbg! {
        as macro "dim" for stream if debug;
        syn::parse_macro_input!(stream as MacroDimTop)
    };
    dim.into_token_stream().into()
}


/// Quantity macro.
///
/// See the crate-level documentation for examples.
#[proc_macro]
pub fn qty(stream: TokenStream) -> TokenStream {
    wrap_dbg!(MacroQty as MacroQtyTop);

    let qty = macro_dbg! {
        as macro "qty" for stream if debug;
        syn::parse_macro_input!(stream as MacroQtyTop)
    };
    qty.into_token_stream().into()
}


#[proc_macro]
pub fn qtype(stream: TokenStream) -> TokenStream {
    wrap_dbg!(MacroQType as MacroQTypeTop);

    let qtype = macro_dbg! {
        as macro "qtype" for stream if debug;
        syn::parse_macro_input!(stream as MacroQTypeTop)
    };
    qtype.into_token_stream().into()
}


#[proc_macro]
pub fn unit(stream: TokenStream) -> TokenStream {
    wrap_dbg!(UnitSpecExpr::as_expr as UnitDefTop);

    let unit = macro_dbg! {
        as macro "unit" for stream if debug;
        syn::parse_macro_input!(stream as UnitDefTop)
    };
    unit.inner.as_expr().into()
}


#[proc_macro]
pub fn utype(stream: TokenStream) -> TokenStream {
    wrap_dbg!(UnitSpecType::as_type as UnitDefTop);

    let dim = macro_dbg! {
        as macro "dim" for stream if debug;
        syn::parse_macro_input!(stream as UnitDefTop)
    };
    dim.inner.as_type().into()
}

#[macro_use]
mod debug;
mod macro_qty;
mod macro_reorg;
mod macro_scale;
mod unit_def;
mod util;

use proc_macro::TokenStream;
use quote::ToTokens;
use macro_qty::MacroQty;
use macro_reorg::Reorg;
use macro_scale::MacroScale;
use unit_def::UnitSpec;


#[proc_macro]
pub fn impl_reorg(stream: TokenStream) -> TokenStream {
    let code = macro_dbg! {
        // as macro "impl_reorg" for stream;
        syn::parse_macro_input!(stream as Reorg)
    };
    code.into_token_stream().into()
}


#[proc_macro]
pub fn impl_scale(stream: TokenStream) -> TokenStream {
    let code = macro_dbg! {
        // as macro "impl_scale" for stream;
        syn::parse_macro_input!(stream as MacroScale)
    };
    code.into_token_stream().into()
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
pub fn unit(stream: TokenStream) -> TokenStream {
    wrap_dbg!(UnitSpec::as_expr as UnitDefTop);

    let unit = macro_dbg! {
        as macro "unit" for stream if debug;
        syn::parse_macro_input!(stream as UnitDefTop)
    };
    unit.inner.as_expr().into()
}


#[proc_macro]
pub fn dim(stream: TokenStream) -> TokenStream {
    wrap_dbg!(UnitSpec::as_type as UnitDefTop);

    let dim = macro_dbg! {
        as macro "dim" for stream if debug;
        syn::parse_macro_input!(stream as UnitDefTop)
    };
    dim.inner.as_type().into()
}

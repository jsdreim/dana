mod debug;
mod macro_qty;
mod macro_reorg;
mod macro_scale;
mod unit_def;

use proc_macro::TokenStream;
use quote::ToTokens;
use macro_qty::MacroQty;
use macro_reorg::Reorg;
use macro_scale::MacroScale;
use unit_def::UnitDef;


#[proc_macro]
pub fn impl_reorg(stream: TokenStream) -> TokenStream {
    let data = syn::parse_macro_input!(stream as Reorg);
    // dbg!(&data.maps);

    // eprintln!("\n");
    // eprintln!("{}", data.to_token_stream());
    // eprintln!("\n");

    data.into_token_stream().into()
}


#[proc_macro]
pub fn impl_scale(stream: TokenStream) -> TokenStream {
    let data = syn::parse_macro_input!(stream as MacroScale);

    // eprintln!("  {}", data.to_token_stream());
    data.into_token_stream().into()
}


/// Quantity macro.
///
/// See the crate-level documentation for examples.
#[proc_macro]
pub fn qty(stream: TokenStream) -> TokenStream {
    // let debug = debug::CallDebug::new("qty", &stream);
    let qty = syn::parse_macro_input!(stream as MacroQty);
    // debug.print(&qty);
    qty.into_token_stream().into()
}


#[proc_macro]
pub fn unit(stream: TokenStream) -> TokenStream {
    let unit = syn::parse_macro_input!(stream as UnitDef);
    let ts = unit.as_value();

    // eprintln!("{ts}");
    ts.into()
}


#[proc_macro]
pub fn utype(stream: TokenStream) -> TokenStream {
    let utype = syn::parse_macro_input!(stream as UnitDef);
    let ts = utype.as_type();

    // eprintln!("{ts}");
    ts.into()
}

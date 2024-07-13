mod macro_qty;
mod macro_reorg;
mod unit_def;

use proc_macro::TokenStream;
use quote::ToTokens;
use macro_qty::MacroQty;
use macro_reorg::Reorg;
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
pub fn qty(stream: TokenStream) -> TokenStream {
    let qty = syn::parse_macro_input!(stream as MacroQty);
    // dbg!(&qty);

    // eprintln!("  {}", qty.to_token_stream());
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

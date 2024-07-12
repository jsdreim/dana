mod macro_reorg;
mod unit_def;

use proc_macro::TokenStream;
use quote::quote;
use macro_reorg::Reorg;


#[proc_macro]
pub fn impl_reorg(stream: TokenStream) -> TokenStream {
    let data = syn::parse_macro_input!(stream as Reorg);
    // dbg!(&data.maps);

    // eprintln!("\n");
    // eprintln!("{}", <Reorg as quote::ToTokens>::to_token_stream(&data));
    // eprintln!("\n");

    quote!(#data).into()
}

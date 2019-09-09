#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;

#[cfg(feature = "export_as_pub")]
mod export_as_pub;

#[cfg(feature = "export_as_pub")]
#[proc_macro_attribute]
pub fn export_as_pub(args: TokenStream, input: TokenStream) -> TokenStream {
    export_as_pub::export_as_pub(args, input)
}

#[cfg(not(feature = "export_as_pub"))]
#[proc_macro_attribute]
pub fn export_as_pub(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[cfg(not(feature = "serde_derive"))]
#[proc_macro_derive(Serialize)]
pub fn serde_serialize(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[cfg(not(feature = "serde_derive"))]
#[proc_macro_derive(Deserialize)]
pub fn serde_deserialize(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

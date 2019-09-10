// Copyright (C) 2019 Alibaba Cloud. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 or BSD-3-Clause

#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;

#[cfg(feature = "export_as_pub")]
mod export_as_pub;
#[cfg(feature = "serde_derive_ffi")]
mod ffi;

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

#[cfg(not(feature = "serde_derive_ffi"))]
#[proc_macro_derive(SerializeFfi)]
pub fn serde_serialize_ffi(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[cfg(feature = "serde_derive_ffi")]
#[proc_macro_derive(SerializeFfi)]
pub fn serde_serialize_ffi(input: TokenStream) -> TokenStream {
    ffi::serialize_ffi(input)
}

#[cfg(not(feature = "serde_derive_ffi"))]
#[proc_macro_derive(DeserializeFfi)]
pub fn serde_deserialize_ffi(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[cfg(feature = "serde_derive_ffi")]
#[proc_macro_derive(DeserializeFfi)]
pub fn serde_deserialize_ffi(input: TokenStream) -> TokenStream {
    ffi::deserialize_ffi(input)
}

#[cfg(not(feature = "serde_derive_ffi"))]
#[proc_macro_derive(DeserializeFfiFam)]
pub fn serde_deserialize_ffi_fam(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[cfg(feature = "serde_derive_ffi")]
#[proc_macro_derive(DeserializeFfiFam)]
pub fn serde_deserialize_ffi_fam(input: TokenStream) -> TokenStream {
    ffi::deserialize_ffi_fam(input)
}

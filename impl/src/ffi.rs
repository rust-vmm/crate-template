// Copyright (C) 2019 Alibaba Cloud. All rights reserved.
// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(super) fn serialize_ffi(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    if ident.to_string() == "__IncompleteArrayField" && input.generics.params.len() == 1 {
        TokenStream::from(quote! {
            impl<T> Serialize for __IncompleteArrayField<T> {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                    where S: Serializer,
                {
                    [0u8; 0].serialize(serializer)
                }
            }
        })
    } else if ident.to_string() == "__BindgenBitfieldUnit" && input.generics.params.len() == 2 {
        TokenStream::from(quote! {
            impl<Storage, Align> Serialize for __BindgenBitfieldUnit<Storage, Align>
                where Storage: AsRef<[u8]> + AsMut<[u8]>,
            {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                    where S: Serializer,
                {
                    let bytes = serialize_ffi::<__BindgenBitfieldUnit<Storage, Align>>(&self);
                    bytes.serialize(serializer)
                }
            }
        })
    } else if input.generics.params.len() == 0 {
        TokenStream::from(quote! {
            impl Serialize for #ident {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                    where
                        S: Serializer,
                {
                    let bytes = serialize_ffi::< #ident >(&self);
                    bytes.serialize(serializer)
                }
            }
        })
    } else {
        panic!("can't derive Serialize implementation for {}", ident);
    }
}

pub(super) fn deserialize_ffi(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    if ident.to_string() == "__IncompleteArrayField" && input.generics.params.len() == 1 {
        TokenStream::from(quote! {
            impl<'de, T> de::Deserialize<'de> for __IncompleteArrayField<T> {
                fn deserialize<D>(_: D) -> std::result::Result<Self, D::Error>
                    where D: de::Deserializer<'de>,
                {
                    Ok(__IncompleteArrayField::new())
                }
            }
        })
    } else if ident.to_string() == "__BindgenBitfieldUnit" && input.generics.params.len() == 2 {
        TokenStream::from(quote! {
            impl<'de, Storage, Align> de::Deserialize<'de> for __BindgenBitfieldUnit<Storage, Align>
                where Storage: AsRef<[u8]> + AsMut<[u8]>,
            {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                    where D: de::Deserializer<'de>,
                {
                    let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
                    Ok(deserialize_ffi::<__BindgenBitfieldUnit<Storage, Align>>(v))
                }
            }
        })
    } else if input.generics.params.len() == 0 {
        TokenStream::from(quote! {
            impl<'de> de::Deserialize<'de> for #ident {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                    where D: de::Deserializer<'de>,
                {
                    let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
                    deserialize_ffi::<#ident>(v)
                        .map_err(|err| D::Error::custom(format_args!("invalid length {}, expected {}", err.1, &err.0)))
                }
            }

            impl SizeofFamStruct for #ident {
                fn size_of(&self) -> usize {
                    std::mem::size_of::<#ident>()
                }
            }
        })
    } else {
        panic!("can't derive Serialize implementation for {}", ident);
    }
}

pub(super) fn deserialize_ffi_fam(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    TokenStream::from(quote! {
        impl<'de> #ident {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Vec<Self>, D::Error>
                where D: de::Deserializer<'de>,
            {
                let v: ByteBuf = ByteBuf::deserialize::<D>(deserializer)?;
                deserialize_ffi_fam::<#ident>(v)
                    .map_err(|err| D::Error::custom(format_args!("invalid length {}, expected {}", err.1, &err.0)))
            }
        }
    })
}

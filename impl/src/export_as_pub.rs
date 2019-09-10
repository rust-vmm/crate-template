// Copyright (C) 2019 Alibaba Cloud. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 or BSD-3-Clause

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{
    parse_macro_input, parse_str, Attribute, AttributeArgs, ItemStruct, ItemUse, Meta, NestedMeta,
    Token, Visibility,
};

pub fn export_as_pub(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let token = parse_macro_input!(input as Token);

    match token {
        Token::Struct(item) => build_struct(item, args),
        Token::Use(item) => build_use(item, args),
    }
}

fn build_struct(mut item: ItemStruct, args: AttributeArgs) -> TokenStream {
    item.vis = parse_str::<Visibility>("pub").unwrap();
    for field in item.fields.iter_mut() {
        if args.is_empty() {
            field.vis = parse_str::<Visibility>("pub").unwrap();
        } else {
            if let Some(ref ident) = field.ident {
                for arg in args.iter() {
                    if let NestedMeta::Meta(Meta::Path(path)) = arg {
                        if let Some(ident2) = path.get_ident() {
                            if ident2.to_string() == ident.to_string() {
                                field.vis = parse_str::<Visibility>("pub").unwrap();
                            }
                        }
                    }
                }
            }
        }
    }

    let stream = quote! {
        #item
    };
    stream.into()
}

fn build_use(mut item: ItemUse, _args: AttributeArgs) -> TokenStream {
    item.vis = parse_str::<Visibility>("pub").unwrap();
    let stream = quote! {
        #item
    };
    stream.into()
}

enum Token {
    Struct(ItemStruct),
    Use(ItemUse),
}

impl Parse for Token {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = Attribute::parse_outer(input)?;

        let ahead = input.fork();
        ahead.parse::<Visibility>()?;
        ahead.parse::<Option<Token![unsafe]>>()?;
        if ahead.peek(Token![struct]) {
            let mut item: ItemStruct = input.parse()?;
            attrs.extend(item.attrs);
            item.attrs = attrs;
            Ok(Token::Struct(item))
        } else if ahead.peek(Token![use]) {
            let mut item: ItemUse = input.parse()?;
            attrs.extend(item.attrs);
            item.attrs = attrs;
            Ok(Token::Use(item))
        } else {
            Err(input.error("expect struct or enum block"))
        }
    }
}

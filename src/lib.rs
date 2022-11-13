#![feature(ptr_internals)]
extern crate proc_macro;

use proc_macro::TokenStream;
use std::borrow::Borrow;
use std::ptr::Unique;

use syn::{parse_macro_input, DeriveInput, Expr, Lit};
use quote::quote;


#[proc_macro]
pub fn unroll(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);

    let mut vec = Vec::new();
    if let Expr::ForLoop(ref f) = input {
        if let Expr::Range(ref rang) = f.clone().expr.borrow() {
            if let Expr::Lit(ref from) = rang.from.as_ref().unwrap().borrow() {
                if let Lit::Int(ref i) = from.clone().lit {
                    let i: i64 = i.base10_parse().unwrap();
                    eprintln!("from wtfbbq {}", i);
                    if let Expr::Lit(ref to) = rang.to.as_ref().unwrap().borrow() {
                        if let Lit::Int(ref j) = to.clone().lit {
                            let j: i64 = j.base10_parse().unwrap();
                            eprintln!("to wtfbbq {}", j);

                            for k in i..20 {
                                vec.push(
                                    f.body.stmts[0].clone()
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // let functions: TokenStream = vec.iter().collect();
    let tokens = quote! {
        #(#vec)*
    };

    tokens.into()
}
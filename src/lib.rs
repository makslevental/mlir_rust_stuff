extern crate proc_macro;

use proc_macro::TokenStream;
use std::borrow::Borrow;
use std::ops::DerefMut;

use quote::quote;
use rustc_ast::{HasTokens, Lit, LitKind};
use rustc_ast::ExprKind;
use rustc_ast::mut_visit::MutVisitor;
use rustc_ast::tokenstream::TokenStreamBuilder;
use rustc_data_structures::map_in_place::MapInPlace;
use syn::Expr;
use syn::parse_macro_input;

use crate::parse::librustc_expr;

mod parse;

#[proc_macro]
pub fn unroll(input: TokenStream) -> TokenStream {
    let mut vec = Vec::new();
    rustc_span::create_default_session_globals_then(|| {
        match librustc_expr(input.to_string().as_str()).unwrap().into_inner().kind {
            ExprKind::ForLoop(_, range, block, _) => {
                let ExprKind::Range(Some(i), Some(j), _) = &range.kind else { panic!(); };
                let ExprKind::Lit(Lit { token_lit, kind: LitKind::Int(i, _k), span }) = &i.kind else { panic!(); };
                let ExprKind::Lit(Lit { token_lit, kind: LitKind::Int(j, _k), span }) = &j.kind else { panic!(); };

                dbg!(&i);
                dbg!(&j);

                for _ in 0..*j {
                    vec.push(block.stmts[0].clone());
                }
            }
            _ => {}
        }
    });

    let input = parse_macro_input!(input as Expr);
    let tokens = quote! {
        #input
    };

    tokens.into()
}
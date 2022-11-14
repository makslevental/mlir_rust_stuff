// extern crate rustc_ast;
// extern crate rustc_parse as parse;
// extern crate rustc_session;
// extern crate rustc_span;

use std::borrow::Borrow;

use rustc_ast::ast;
use rustc_ast::ptr::P;
use rustc_parse as parse;
use rustc_session::parse::ParseSess;
use rustc_span::FileName;

macro_rules! errorf {
    ($($tt:tt)*) => {{
        use ::std::io::Write;
        let stderr = ::std::io::stderr();
        write!(stderr.lock(), $($tt)*).unwrap();
    }};
}

pub fn librustc_expr(input: &str) -> Option<P<ast::Expr>> {
    let sess = ParseSess::with_silent_emitter(None);
    let e = parse::new_parser_from_source_str(
        &sess,
        FileName::Custom("test_precedence".to_string()),
        input.to_string(),
    )
        .parse_expr();
    match e {
        Ok(expr) => Some(expr),
        Err(mut diagnostic) => {
            diagnostic.emit();
            None
        }
    }
}

pub fn syn_expr(input: &str) -> Option<syn::Expr> {
    match syn::parse_str(input) {
        Ok(e) => Some(e),
        Err(msg) => {
            errorf!("syn failed to parse\n{:?}\n", msg);
            None
        }
    }
}

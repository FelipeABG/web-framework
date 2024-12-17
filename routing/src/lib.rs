extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parser, parse_macro_input, punctuated::Punctuated, token::Comma, Expr, ItemFn, Lit,
};

#[proc_macro_attribute]
pub fn route(attr: TokenStream, function: TokenStream) -> TokenStream {
    let fn_ast = parse_macro_input!(function as ItemFn);
    let attr_ast = Punctuated::<Expr, Comma>::parse_terminated
        .parse(attr)
        .unwrap();

    let method = match attr_ast.iter().next().unwrap() {
        Expr::Path(path) => path
            .path
            .segments
            .clone()
            .into_iter()
            .next()
            .unwrap()
            .ident
            .to_string(),
        _ => panic!("Incorrect argument"),
    };

    let path = match attr_ast.iter().skip(1).next().unwrap() {
        Expr::Lit(exp) => match exp.lit.clone() {
            Lit::Str(lit_str) => lit_str
                .token()
                .to_string()
                .chars()
                .filter(|c| *c != '"')
                .collect::<String>(),
            _ => panic!("Incorrect argument"),
        },
        _ => panic!("Incorrect argument"),
    };

    println!("{:#?}", method);
    println!("{:#?}", path);

    let expanded = quote! {
        #fn_ast
    };

    TokenStream::from(expanded)
}

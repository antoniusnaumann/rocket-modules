#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote};
use syn::{ItemMod, Item::Fn, ItemFn, Ident, Item, ItemConst, ExprArray, ExprLit, LitStr, Expr};
use syn::Lit::Str;
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[proc_macro]
pub fn module(input: TokenStream) -> TokenStream {
    let _routes = parse_macro_input!(input as Ident);

    todo!("Retrieve method names from module...")
}

#[proc_macro_attribute]
pub fn route_module(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as ItemMod);
    let content = module.content
        .expect("Currently, only modules which declare their content in-place ('mod my_module { ... }') are supported!");

    let routes = content.1.iter()
        .filter_map(|item| if let Fn(func) = item { Some(func) } else { None })
        .filter(|&func| is_rocket_route(func))
        .map(|route| route.sig.ident.to_string())
        .collect::<Vec<_>>();

    let routes_len = routes.len();
    let mut elems = Punctuated::new();

    for route in routes {
        let lit = Str(LitStr::new(route.as_str(), Span::call_site()));
        elems.push(Expr::from(ExprLit { attrs: vec![], lit }));
    }

    let route_literal = ExprArray {
        attrs: vec![],
        bracket_token: Default::default(),
        elems,
    };

    let fn_routes = TokenStream::from(quote! {
        pub const __ROUTES: [&str; #routes_len] = #route_literal;
    });

    let mut items = content.1.clone();
    items.push(Item::from(parse_macro_input!(fn_routes as ItemConst)));

    let module = ItemMod {
        content: Some((content.0.clone(), items)),
        ..module
    };

    TokenStream::from(quote!(#module))
}

fn is_rocket_route(func: &ItemFn) -> bool {
    func.attrs
        .iter()
        .any(|attr|
            attr.path.segments.iter()
            .any(|s|
                ROCKET_ROUTE_KEYWORDS.iter()
                .any(|&keyword| keyword == s.ident.to_string())) )
}

const ROCKET_ROUTE_KEYWORDS: [&str; 8] = [
    "route",
    "get",
    "post",
    "put",
    "delete",
    "head",
    "patch",
    "options"
];

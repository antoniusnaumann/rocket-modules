#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{ExprMethodCall, ItemMod, Item::Fn, ItemFn, Item, ExprArray, Path};
use syn::Expr::MethodCall;
use syn::punctuated::Punctuated;


#[proc_macro]
pub fn module(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as Path);
    let module = &path.segments.last().unwrap().ident;

    TokenStream::from(quote!(#module::__routes()))
}

#[proc_macro_attribute]
pub fn route_module(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as ItemMod);
    let content = module.content
        .expect("Currently, only modules which declare their content in-place ('mod my_module { ... }') are supported!");

    let routes = content.1.iter()
        .filter_map(|item| if let Fn(func) = item { Some(func) } else { None })
        .filter(|&func| is_rocket_route(func))
        .map(|route| route.sig.ident.clone())
        .collect::<Vec<_>>();

    let routes_len = routes.len();
    let mut elems = Punctuated::new();

    for route in routes {
        let route_expr = TokenStream::from(quote!(#route { }.into_route()));
        let call = MethodCall(parse_macro_input!(route_expr as ExprMethodCall));
        elems.push(call);
    }

    let route_literal = ExprArray {
        attrs: vec![],
        bracket_token: Default::default(),
        elems,
    };

    let fn_routes = TokenStream::from(quote! {
        pub fn __routes() -> [rocket::Route; #routes_len] {
            #route_literal
        }
    });

    let mut items = content.1.clone();
    items.push(Item::from(parse_macro_input!(fn_routes as ItemFn)));

    let module = ItemMod {
        content: Some((content.0, items)),
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
                .any(|&keyword| s.ident == keyword)))
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

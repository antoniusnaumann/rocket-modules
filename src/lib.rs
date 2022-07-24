#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{ItemMod, Item::Fn, ItemFn, Ident, Item};

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
        .map(|route| format!("{}\n", route.sig.ident))
        .collect::<String>();

    let fn_routes = TokenStream::from(quote! {
        pub fn __fn_routes__() -> Vec<String> {
            #routes
                .split('\n')
                .collect::<Vec<_>>()
                .iter()
                .map(|e| e.to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
    });

    let mut items = content.1.clone();
    items.push(Item::from(parse_macro_input!(fn_routes as ItemFn)));

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

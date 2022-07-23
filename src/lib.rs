#[macro_use]
extern crate syn;

use proc_macro::{TokenStream};
use quote::{quote};
use syn::{ItemMod, Item::Fn, ItemFn};

#[proc_macro_attribute]
pub fn route_module(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as ItemMod);
    let content = module.content.expect("Currently, only modules which declare their content in-place ('mod my_module { ... }') are supported!");
    // TODO: Filter attributes for rocket http method attributes
    let routes = content.1
        .iter()
        .filter_map(|item| if let Fn(func) = item { Some(func) } else { None })
        .filter(|&func| is_rocket_route(func))
        .map(|route| format!("/{}/{}\n", module.ident, route.sig.ident))
        .collect::<String>();

    TokenStream::from(quote! { 
        pub fn __print_routes__() {
            println!("\nRoutes\n");
            println!("{}", #routes);
        }
    })
}

fn is_rocket_route(func: &ItemFn) -> bool {
    func.attrs
        .iter()
        .any(|attr| attr.path.segments
            .iter()
            .any(|s| ROCKET_ROUTE_KEYWORDS.iter().any(|&keyword| keyword == s.ident.to_string())) )
}

const ROCKET_ROUTE_KEYWORDS: [&str; 8] = ["route", "get", "post", "put", "delete", "head", "patch", "options"];

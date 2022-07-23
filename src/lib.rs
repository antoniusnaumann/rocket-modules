#[macro_use]
extern crate syn;

use proc_macro::{TokenStream};
use quote::{quote};
use syn::{ItemMod, Item::Fn};

#[proc_macro_attribute]
pub fn route_module(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let module = parse_macro_input!(input as ItemMod);
    let content = module.content.expect("Currently, only modules which declare their content in-place ('mod my_module { ... }') are supported!");
    // TODO: Filter attributes for rocket http method attributes
    let routes = content.1
        .iter()
        .filter_map(|item| if let Fn(func) = item { Some(func) } else { None })
        .map(|route| format!("{}/{}", module.ident, route.sig.ident))
        .collect::<String>(); 

    TokenStream::from(quote! { 
        pub fn print_routes() {
            println!("{}", #routes);
        }
    })
}

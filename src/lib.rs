//! A small crate that adds macros to conveniently organize [Rocket](rocket.rs) route handlers in modules.
//!
//! This crate is not directly associated with the rocket project, although it is built upon it.
//!
//! # Usage
//! To use `rocket_modules`, add it to your dependencies in your `Cargo.toml`.
//!
//! You should also add a dependency to `rocket` (version `0.5.0-rc2` or higher) if not already present:
//! ```
//! [dependencies]
//! rocket = "0.5.0-rc.2"
//! rocket_modules = "0.1.0"
//! ```

#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{ExprMethodCall, ItemMod, Item::Fn, ItemFn, Item, ExprArray, Path};
use syn::Expr::MethodCall;
use syn::punctuated::Punctuated;

/// Generates a [`Vec<rocket::Route>`](rocket::Route) which contains all functions marked with a [rocket route attribute](rocket::get). The module has to be marked with [`#[route_module]`](macro@route_module).
///
/// # Examples
/// Comparison with rocket's [`routes!`](rocket::routes) macro:
/// ```
/// #[route_module]
/// mod articles {
///     #[get("/")]
///     pub fn all_articles() { /* ... */ }
///
///     #[get("/<_id>")]
///     pub fn get_article(_id: &str) { /* ... */ }
///
///     #[post("/<_id>")]
///     pub fn post_article(_id: &str) { /* ... */ }
///
///     #[route(PATCH, uri = "/<_id>")]
///     pub fn patch_article(_id: &str) { /* ... */ }
/// }
///
/// let routes = routes![articles::all_articles, articles::get_article, articles::post_article, articles::patch_article];
/// let module = module!(articles);
/// assert_routes_eq!(routes, module);
/// ```
///
/// # Usage
/// ```
/// #[route_module]
/// mod articles {
///     #[get("/")]
///     pub fn all_articles() { /* ... */ }
///
///     #[get("/<_id>")]
///     pub fn get_article(_id: &str) { /* ... */ }
///
///     #[post("/<_id>")]
///     pub fn post_article(_id: &str) { /* ... */ }
///
///     #[route(PATCH, uri = "/<_id>")]
///     pub fn patch_article(_id: &str) { /* ... */ }
/// }
///
/// #[launch]
/// fn rocket() -> _ {
///     rocket::build()
///       .mount("/articles", module!(articles))
/// }
/// ```
#[proc_macro]
pub fn module(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as Path);

    TokenStream::from(quote!(#path::__routes()))
}

/// Marks a module as route module which allows it to be passed as an argument to the [`module!`] macro.
///
/// # Usage
/// ```
/// #[route_module]
/// mod articles {
///     #[get("/")]
///     pub fn all_articles() { /* ... */ }
///
///     #[get("/<_id>")]
///     pub fn get_article(_id: &str) { /* ... */ }
///
///     #[post("/<_id>")]
///     pub fn post_article(_id: &str) { /* ... */ }
///
///     #[route(PATCH, uri = "/<_id>")]
///     pub fn patch_article(_id: &str) { /* ... */ }
/// }
/// ```
///
/// # Panics
/// This macro can only be applied to complete module definitions (`mod my_module { ... }`).
///
/// Trying to add this attribute to a module declaration without a body will result this macro to panic at compile-time.
///
/// # Code generation
/// This macro generates `pub fn __routes() -> [rocket::Route; #routes_len] { /* ... */ }` in your module.
/// Do not use a function with this name in your module when applying the [macro@route_module] attribute as this will lead to name clashes.
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
        #[doc(hidden)]
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
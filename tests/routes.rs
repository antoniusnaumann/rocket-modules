#[macro_use]
extern crate route_modules;

#[macro_use]
extern crate rocket;

fn str(route: &rocket::Route) -> String { route.name.as_ref().unwrap().to_string() }
macro_rules! assert_routes_eq {
    ($routes: ident, $module: ident) => {
        // Make sure all routes generated with routes! macro are present in module! macro output as well
        assert!($routes.iter().all(|r| $module.iter().any(|other| str(r) == str(other))));
        // Make sure all routes generated with module! would be generated when using routes! macro
    };
}

#[route_module]
mod articles {
    #[get("/")]
    pub fn _all() {}

    #[get("/<_id>")]
    pub fn _get_with_id(_id: &str) {}

    #[post("/<_id>")]
    pub fn _post_with_id(_id: &str) {}

    #[route(PATCH, uri = "/<_id>")]
    pub fn _patch_with_id(_id: &str) {}

    pub fn no_route() -> bool { true }

    pub fn no_route_too() {}
}

#[test]
fn test_route_number() {
    assert_eq!(articles::__routes().len(),  4);

    // Ensure functions are still there and did not got replaced by macro
    assert_eq!(articles::no_route(), true);
    assert_eq!(articles::no_route_too(), ());
}

#[test]
fn test_generated_route_fn() {
    assert!(articles::__routes().iter().any(|route| str(route) == "_all"));
}

#[test]
fn test_module_macro() {
    assert_eq!(articles::__routes().len(), module!(articles).len());

    let routes = routes![articles::_all, articles::_get_with_id, articles::_post_with_id, articles::_patch_with_id];
    let module = module!(articles);
    assert_eq!(routes.len(), module.len());

    assert_routes_eq!(routes, module);
}

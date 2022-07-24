#[macro_use]
extern crate route_modules;

#[macro_use]
extern crate rocket;

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
    assert_eq!(articles::__ROUTES.len(),  4);

    // Ensure functions are still there and did not got replaced by macro
    assert_eq!(articles::no_route(), true);
    assert_eq!(articles::no_route_too(), ());
}

#[test]
fn test_generated_route_fn() {
    assert!(articles::__ROUTES.iter().any(|&s| s == "_all"));
    assert!(articles::__ROUTES.iter().any(|&s| s == "_patch_with_id"));
    assert!(!articles::__ROUTES.iter().any(|&s| s == "_no_route"));
}

#[test]
fn test_module_macro() {

}
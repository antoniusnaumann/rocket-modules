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

    pub fn _no_route() -> bool { true }

    pub fn _no_route_too() {}
}

#[test]
fn test_route_number() {
    assert_eq!(articles::__fn_routes__().len(),  4);
    assert_eq!(articles::_no_route(), true);
}

#[test]
fn test_generated_route_fn() {
    assert!(articles::__fn_routes__().iter().any(|s| s == "_all"));
    assert!(articles::__fn_routes__().iter().any(|s| s == "_patch_with_id"));
    assert!(!articles::__fn_routes__().iter().any(|s| s == "_no_route"));
}

#[test]
fn test_module_macro() {

}
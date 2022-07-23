#[macro_use]
extern crate route_modules;

#[route_module]
mod articles {
    #[get("/")]
    pub fn all() {}

    #[get("/<id>")]
    pub fn get_with_id(id: &str) {}

    #[post("/<id>")]
    pub fn post_with_id(id: &str) {}

    #[route(PATCH, "/<id>")]
    pub fn patch_with_id(id: &str) {}

    pub fn no_route() {}

    #[no_route]
    pub fn no_route_too() {}
}

#[test]
fn test_route_number() {
    assert_eq!(__routes__().len(),  4)
}
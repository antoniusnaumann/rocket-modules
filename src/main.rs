use route_modules::*;

#[macro_use]
extern crate rocket;

#[route_module]
mod articles {
    #[get("/")]
    pub fn all() {
    }

    #[get("/<id>")]
    pub fn with_id(id: &str) {
    }

    pub fn no_route() {

    }
}

fn main() {
    __print_routes__();
}

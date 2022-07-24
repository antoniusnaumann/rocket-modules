# Route Modules
A small crate that adds macros to conveniently organize [Rocket](rocket.rs) route handlers in modules.

## Example
Instead of explicitly stating all routes that should be mounted...
```Rust
#[get("/")]
pub fn all_articles() {}

#[get("/<_id>")]
pub fn get_article(_id: &str) {}

#[post("/<_id>")]
pub fn post_article(_id: &str) {}

#[route(PATCH, uri = "/<_id>")]
pub fn patch_article(_id: &str) {}

#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/articles", routes![all_articles, get_article, post_article, patch_article])
}
```

...this crate allows you to write the following:

```Rust
#[route_module]
mod articles {
  // Same code as above
  ...
}

#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/articles", module!(articles))
}
```

## Installation
**TODO:** This crate has yet to uploaded to crates.io. Will happen soon! :)

*Note: Compatability of this crate was testes with Rocket version "0.5.0-rc.2", it may or may not work with previous version.*

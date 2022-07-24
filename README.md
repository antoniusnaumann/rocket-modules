# Rocket Modules
[![Build Status](https://github.com/antoniusnaumann/rocket-modules/actions/workflows/build.yml/badge.svg)](https://github.com/antoniusnaumann/rocket-modules/actions)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-F46623)](https://www.rust-lang.org)
[![Crates.io Version](https://img.shields.io/crates/v/rocket_modules.svg)](https://crates.io/crates/rocket_modules)

A small crate that adds macros to conveniently organize [Rocket](https://rocket.rs) route handlers in modules. This crate is not directly associated with the rocket project, although it is built upon it.

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
To install `rocket_modules`, add it to your dependencies in your `Cargo.toml`.

You should also add a dependency to `rocket` (version `0.5.0-rc2` or higher) if not already present:
```TOML
[dependencies]
rocket = "0.5.0-rc.2"
rocket_modules = "0.1.0"
```

*Note: Compatability of this crate was tested with Rocket version "0.5.0-rc.2", it may or may not work with previous versions.*

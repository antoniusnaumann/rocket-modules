# Rocket Modules
[![Build Status](https://github.com/antoniusnaumann/rocket-modules/actions/workflows/build.yml/badge.svg)](https://github.com/antoniusnaumann/rocket-modules/actions)
[![Publish Status](https://github.com/antoniusnaumann/rocket-modules/actions/workflows/publish.yml/badge.svg)](https://github.com/antoniusnaumann/rocket-modules/actions)
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

## License
This code is dual-licensed and availabe under MIT-License or Apache 2.0-License depending on what suits your needs best.

### Apache 2.0
```
Copyright 2022 Antonius Naumann

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
### MIT
```
MIT License

Copyright (c) 2022 Antonius Naumann

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

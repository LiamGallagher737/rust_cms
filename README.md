<div align="center">

# Rust CMS

[<img alt="build status" src="https://img.shields.io/github/workflow/status/LiamGallagher737/rust_cms/Rust/master?style=for-the-badge" height="24">](https://github.com/LiamGallagher737/rust_cms/actions)
[<img alt="github" src="https://img.shields.io/badge/github-rust_cms-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="24">](https://github.com/LiamGallagher737/rust_cms)
<!-- [<img alt="crates.io" src="https://img.shields.io/crates/v/rust_cms.svg?style=for-the-badge&color=fc8d62&logo=rust" height="24">](https://crates.io/crates/rust_cms) -->
<!-- [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rust_cms-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="24">](https://docs.rs/rust_cms) -->

</div>

## Example

```rust
#[macro_use] extern crate rocket;
use rust_cms::prelude::*;
use serde::{Deserialize, Serialize};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![get_document, post_document])
}

documents!(Person, Restaurant);

models! {
    struct Person {
        name: String,
        age: u32,
        gender: Gender,
    },
    enum Gender {
        Male,
        Female,
        Other,
    },
    struct Restaurant {
        name: String,
        rating: u32,
    },
}
```
This will result in the following endpoints

### Get
- `/api/document/person/<id>`
- `/api/restaurant/person/<id>`

### Post
- `/api/document/person`
- `/api/restaurant/person`

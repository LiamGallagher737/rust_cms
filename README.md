# Rust CMS

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

#[derive(Model, Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    age: u32,
    gender: Gender,
}

#[derive(Model, Serialize, Deserialize, Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Model, Serialize, Deserialize, Debug)]
pub struct Restaurant {
    name: String,
    rating: u32,
}
```
This will result in the following endpoints

### Get
- `/api/document/person/<id>`
- `/api/restaurant/person/<id>`

### Post
- `/api/document/person`
- `/api/restaurant/person`

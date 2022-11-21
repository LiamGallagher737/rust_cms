use std::vec;

use super::*;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn post_endpoint() {
    // Name: "KFC", Rating: 4
    let restaurant_bytes = vec![
        0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4B, 0x46, 0x43, 0x04, 0x00, 0x00, 0x00,
    ];
    let client = Client::tracked(rocket()).unwrap();
    let response = client
        .post(uri!("/api/document/restaurant"))
        .body(restaurant_bytes)
        .header(ContentType::Bytes)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

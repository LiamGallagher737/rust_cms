#[get("/document/<doctype>/<id>")]
pub fn get_document(doctype: String, id: usize) -> String {
    "h".into()
}

#[post("/document/<doctype>")]
pub fn post_document(doctype: &str) -> Option<String> {
    Some("h".into())
}

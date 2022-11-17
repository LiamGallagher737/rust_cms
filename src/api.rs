use rocket::State;

use crate::Doctypes;

#[get("/document/<doctype>/<id>")]
pub fn get_document(doctype: String, id: usize) -> String {
    "h".into()
}

#[post("/document/<doctype>")]
pub fn post_document(doctypes: &State<Doctypes>, doctype: &str) -> Option<String> {
    let schema_data = doctypes.get(doctype)?;
    Some("h".into())
}

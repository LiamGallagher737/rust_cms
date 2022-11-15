use rocket_dyn_templates::{context, Template};

#[get("/login")]
pub fn login_page() -> Template {
    Template::render("login", context! {})
}

use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde_json::json;

use crate::models::{User};
use crate::database;

#[get("/")]
pub fn home() -> Template {
    let context = json!({});
    Template::render("home", &context)
}

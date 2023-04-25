use crate::models::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct Context {
    pub title: String,
    pub user: User,
}

// Implement the render function for the Profile view
pub fn render(user: User) -> String {
    let context = Context {
        title: "Profile".to_string(),
        user,
    };

    let tera = compile_templates!("templates/**/*");
    tera.render("profile", &context).unwrap()
}

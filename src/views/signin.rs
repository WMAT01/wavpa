// Signin route handling

use rocket::request::{Form, FlashMessage};
use rocket::response::{Redirect, Flash};
use rocket_contrib::templates::Template;
use crate::models::User;
use std::collections::HashMap;

// Form data for user signin
#[derive(FromForm)]
pub struct SigninForm {
    email: String,
    password: String,
}

#[post("/signin", data = "<form>")]
pub fn signin(form: Form<SigninForm>, flash: Option<FlashMessage>) -> Result<Flash<Redirect>, Template> {
    let user_data = form.into_inner();
    // TODO: Add validation for user input
    
    // TODO: Verify user credentials and check if the email is confirmed
    let user = User::get_by_email(&user_data.email);
    if let Some(user) = user {
        if user.is_confirmed && user.verify_password(&user_data.password) {
            // Create a new session and database for the user
            let session = create_session(&user.id.to_string());
            create_database(&session.id.to_string());

            return Ok(Flash::success(Redirect::to(uri!(crate::routes::session::session)), "Signed in successfully."));
        }
    }
    
    // Show an error message if signin fails
    let mut context = HashMap::new();
    if let Some(msg) = flash {
        context.insert("flash", msg.msg());
    }
    context.insert("signin_failed", "Invalid email or password, or email not confirmed.");
    Err(Template::render("signin", context))
}

#[get("/signin")]
pub fn signin_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }
    Template::render("signin", context)
}

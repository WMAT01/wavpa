// Signup route handling

use rocket::request::{Form, FlashMessage};
use rocket::response::{Redirect, Flash};
use crate::email::send_confirmation_email;
use crate::models::User;

use validator::{Validate, ValidationError};

#[derive(Debug, FromForm, Validate)]
pub struct SignupForm {
//    #[validate(email)]
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
//    #[validate(length(min = 8))]
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub name: String,
}

// ...

/**
// Form data for user signup
#[derive(FromForm)]
pub struct SignupForm {
    email: String,
    password: String,
    // Add other fields as required, such as username, first name, last name, etc.
}
**/

/**
#[post("/signup", data = "<form>")]
pub fn signup(form: Form<SignupForm>, flash: Option<FlashMessage>) -> Result<Flash<Redirect>, Redirect> {
    let user_data = form.into_inner();
    // TODO: Add validation for user input
    
    // TODO: Save the new user to the database and generate the confirmation token
    let user = User::new(user_data.email.clone(), user_data.password);
    let confirmation_token = user.generate_confirmation_token();
    
    // TODO: Send confirmation email to the user
    if let Err(e) = send_confirmation_email(&user_data.email, &confirmation_token) {
        return Err(Redirect::to(uri!(crate::routes::signin::signin)));
    }
    
    // Redirect to the signin page with a success message
    Ok(Flash::success(Redirect::to(uri!(crate::routes::signin::signin)), "Account created. Check your email for a confirmation link."))
}
**/
/**#[post("/signup", data = "<form>")]
pub fn signup(form: Form<SignupForm>, flash: Option<FlashMessage>) -> Result<Flash<Redirect>, Redirect> {
    let user_data = form.into_inner();

    // TODID: Add validation for user input
/**    if let Err(validation_errors) = user_data.validate() {
        // Handle validation errors, show them in the template, and return an error
    }

    // Validate user input
    if let Err(validation_errors) = user_data.validate() {
        let mut context = HashMap::new();
        if let Some(msg) = flash {
            context.insert("flash", msg.msg());
        }
        context.insert("validation_errors", &validation_errors);
        return Err(Template::render("signup", context));
    }
    // Save the new user to the database
    let user = User::new(user_data.email.clone(), user_data.password);
    let db = SQLite3; // Assuming SQLite3 as the database, update as needed
    if let Err(e) = user.save(&db) {
        return Err(Redirect::to(uri!(crate::routes::signup::signup)));
    }

    // Send the confirmation email to the user
    if let Err(e) = send_confirmation_email(&user_data.email, &user.confirmation_token) {
        return Err(Redirect::to(uri!(crate::routes::signup::signup)));
    }

    // Redirect to the signin page with a success message
    Ok(Flash::success(Redirect::to(uri!(crate::routes::signin::signin)), "Account created. Check your email for a confirmation link."))
}
**/
/**#[post("/signup", data = "<form>")]
pub fn signup(form: Form<SignupForm>, flash: Option<FlashMessage>) -> Result<Flash<Redirect>, Template> {
    let user_data = form.into_inner();

    // Validate user input
    if let Err(validation_errors) = user_data.validate() {
        let mut context = HashMap::new();
        if let Some(msg) = flash {
            context.insert("flash", msg.msg());
        }
        context.insert("validation_errors", &validation_errors);
        return Err(Template::render("signup", context));
    }

    // Create a new user with the provided information
    let user = User::new(
        user_data.email.clone(),
        user_data.password.clone(),
        user_data.name.clone(),
    );

    // TODO: Insert the user into the database and handle any errors

    // Verify if the user is confirmed and if their password is correct
    if user.is_confirmed && user.verify_password(&user_data.password) {
        return Ok(Flash::success(
            Redirect::to(uri!(crate::routes::session::new)),
            format!("Successfully signed up with {}!", user.provider),
        ));
    }

    // Show an error message if signup fails
    let mut context = HashMap::new();
    if let Some(msg) = flash {
        context.insert("flash", msg.msg());
    }
    context.insert("signup_failed", "Failed to sign up. Please try again.");
    Err(Template::render("signup", context))
}
**/
#[post("/signup", data = "<form>")]
pub fn signup(form: Form<SignupForm>, flash: Option<FlashMessage>) -> Result<Flash<Redirect>, Template> {
    let user_data = form.into_inner();

    // Validate user input
    if let Err(validation_errors) = user_data.validate() {
        let mut context = HashMap::new();
        if let Some(msg) = flash {
            context.insert("flash", msg.msg());
        }
        context.insert("validation_errors", &validation_errors);
        return Err(Template::render("signup", context));
    }

    // Create a new user with the provided information
    let user = User::new(
        user_data.email.clone(),
        user_data.password.clone(),
        user_data.name.clone(),
    );

    // Save the new user to the database
    let db = SQLite3; // Assuming SQLite3 as the database, update as needed
    if let Err(e) = user.save(&db) {
        return Err(Redirect::to(uri!(crate::routes::signup::signup)));
    }

    // Send the confirmation email to the user
    if let Err(e) = send_confirmation_email(&user_data.email, &user.confirmation_token) {
        return Err(Redirect::to(uri!(crate::routes::signup::signup)));
    }

    // Verify if the user is confirmed and if their password is correct
    if user.is_confirmed && user.verify_password(&user_data.password) {
        return Ok(Flash::success(
            Redirect::to(uri!(crate::routes::session::new)),
            format!("Successfully signed up with {}!", user.provider),
        ));
    }

    // Show an error message if signup fails
    let mut context = HashMap::new();
    if let Some(msg) = flash {
        context.insert("flash", msg.msg());
    }
    context.insert("signup_failed", "Failed to sign up. Please try again.");
    Err(Template::render("signup", context))
}



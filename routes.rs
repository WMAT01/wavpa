// Routes module
// Import the necessary modules
/**
use crate::views::{profile, billing};

// Import HashMap at the beginning of the file
use std::collections::HashMap;

// ...
// ...
pub mod home;
pub mod signin;
pub mod signup;
pub mod session;
pub mod profile;
pub mod billing;

// Each submodule should contain the route handling for a specific page.
// For example, the home submodule will have the route for the index page.
// This helps keep the route handling code organized and easy to maintain.

// Profile route

#[get("/profile")]
pub fn profile(user: User) -> Template {
    let context = profile::Context {
        title: "Profile".to_string(),
        user,
    };
    Template::render("profile", &context)
}

// Billing route
#[get("/billing")]
pub fn billing(user: User) -> Template {
    let context = billing::Context {
        title: "Billing".to_string(),
        user,
    };
    Template::render("billing", &context)
}
**/

// Inside the profile route handler
/**
#[get("/profile")]
fn profile(user: User) -> Template {
    let mut context = Context::new();
    context.insert("user", &user);

    // Convert user.preferences to HashMap and insert it into the context
    let preferences_map: HashMap<String, String> = user.preferences.into();
    context.insert("preferences", &preferences_map);

    Template::render("profile", &context)
}

// Inside the billing route handler
#[get("/billing")]
fn billing(user: User) -> Template {
    let mut context = Context::new();
    context.insert("user", &user);

    // Convert user.billing_details to HashMap and insert it into the context
    let billing_details_map: HashMap<String, String> = user.billing_details.into();
    context.insert("billing_details", &billing_details_map);

    Template::render("billing", &context)
}
**/
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod home;
mod session;
mod signin;
mod signup;
mod profile;
mod billing;

use rocket_contrib::templates::Template;
use home::home;
use session::session;
use signin::signin;
use signup::signup;
use profile::profile;
use billing::billing;

#[launch]
fn rocket() -> _ {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![
            home,
            session,
            signin,
            signup,
            profile,
            billing,
        ])
}

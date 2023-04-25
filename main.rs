#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod database;
mod email;
mod models;
mod routes;
mod utils;
mod views;

use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::path::{Path, PathBuf};

use routes::*;

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![
            static_files,
            home::index,
            signin::signin,
            signup::signup,
            session::session,
            profile::profile,
            billing::billing
        ])
        .launch();
}

/**

Set up the main function in main.rs and initialize the server with routes.
Create the database models in models.rs, using traits for generic database interaction.
Set up the database connections and the logic for SQLite3, SurrealDB, and Apache Kafka in database.rs.
Write the email daemon logic in email.rs.
Implement the routes in routes.rs to handle navigation between pages.
Write helper functions in utils.rs for tasks such as authentication, third-party sign-ins, and other shared operations.
Create the view components in the views folder (e.g., home.rs, signup.rs, signin.rs, etc.) using the Yew library.
Design the HTML templates in the templates folder for each corresponding view component.
Implement the third-party payment integrations on the billing page.


**/

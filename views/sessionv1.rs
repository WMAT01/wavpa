use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde_json::json;
use rocket_contrib::databases::rusqlite::Connection;

use crate::models::{User, Session};
use crate::database;



#[get("/session")]
pub fn session(user: User) -> Template {
    let context = json!({
        "user": user,
        "session": Session::load_most_recent(user.id),
    });
    Template::render("session", &context)
}

// This function creates a new session and returns the session object.
fn create_session(user_id: &str) -> Session {
    let new_session = Session::new(user_id);
    // Save the session to the database.
    new_session.save();
    new_session
}

// This function creates a new SQLite3 database for the session.
fn create_database(session_id: &str) {
    let db_path = format!("databases/{}.db", session_id);
    let _conn = Connection::open(&db_path).expect("Failed to create SQLite3 database");

    // TODO: Create tables and set up the database schema.

    println!("Created SQLite3 database at {}", db_path);
}

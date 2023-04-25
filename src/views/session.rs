use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde_json::json;
use rocket_contrib::databases::rusqlite::Connection;

use crate::models::{User, Session};
use crate::database;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub start_date: chrono::DateTime<Utc>,
    pub end_date: Option<chrono::DateTime<Utc>>,
    pub nickname: Option<String>,
    pub description: Option<String>,
    pub database_type: DatabaseType,
    pub database_path: String,
    pub xml_description: String,
    pub currently_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseType {
    SQLite3,
    SurrealDB,
    ApacheKafka,
}

impl Session {
    pub fn new(user_id: &str) -> Self {
        let id = Uuid::new_v4();
        let start_date = Utc::now();
        let database_type = DatabaseType::SQLite3;

        Session {
            id,
            user_id: Uuid::parse_str(user_id).unwrap(),
            start_date,
            end_date: None,
            nickname: None,
            description: None,
            database_type,
            database_path: format!("databases/{}.db", id),
            xml_description: String::new(),
            currently_active: true,
        }
    }

    // TODO: Implement save() to save a session in the database
    pub fn save(&self, db: &impl SessionDatabase) -> Result<(), String> {
        db.save_session(self)
    }
}

pub trait SessionDatabase {
    fn save_session(&self, session: &Session) -> Result<(), String>;
}

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

 use chrono::prelude::*;
use rand::Rng;
use uuid::Uuid;
use crate::database::{Database, SQLite3}; // Assuming SQLite3 as the database, update as needed

// Define the User struct
pub struct User {
    id: Uuid,
    email: String,
    password: String,
    phone_number: Option<String>,
    confirmed: bool,
    signed_up: DateTime<Utc>,
    confirmation_token: String,
    preferences: Preferences,
    billing_details: BillingDetails,
    sessions: Vec<Session>,
}

// Define the Preferences struct
pub struct Preferences {
    // Add preferences fields
    bool1: bool,
    bool2: bool,
    bool3: bool,
    float1: f32,
    float2: f32,
    float3: f32,
    string1: String,
    string2: String,
    string3: String,
    database_type: DatabaseType,
}

// Define the BillingDetails struct
pub struct BillingDetails {
    // Add billing details fields
    // TODO: Define the fields for billing details
}

// Define the Session struct
pub struct Session {
    id: Uuid,
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
    nickname: Option<String>,
    description: Option<String>,
    database_type: DatabaseType,
    database_path: String,
    xml_file: String,
    currently_active: bool,
}

// Define the DatabaseType enum
pub enum DatabaseType {
    SQLite3,
    SurrealDB, // TODO: Add SurrealDB details
    ApacheKafka, // TODO: Add Apache Kafka details
}

// Implement methods for the User struct
impl User {
    // Implement methods related to user data
    pub fn new(email: String, password: String, phone_number: Option<String>, preferences: Preferences, billing_details: BillingDetails) -> Self {
        User {
            id: Uuid::new_v4(),
            email,
            password,
            phone_number,
            confirmed: false,
            signed_up: Utc::now(),
            confirmation_token: Self::generate_confirmation_token(),
            preferences,
            billing_details,
            sessions: Vec::new(),
        }
    }

    // Generate a confirmation token for the user
    fn generate_confirmation_token() -> String {
        let mut rng = rand::thread_rng();
        let token: String = rng.sample_iter(&rand::distributions::Alphanumeric).take(32).map(char::from).collect();
        token
    }

    // Save the user to the database
    pub fn save(&self, db: &impl Database) -> Result<(), String> {
        db.save_user(self)
    }
}


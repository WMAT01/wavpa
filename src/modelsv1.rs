// Models module

use chrono::prelude::*;
use rand::Rng;
use crate::database::{Database, SQLite3}; // Assuming SQLite3 as the database, update as needed

// Define the User struct
pub struct User {
    // Add user fields such as id, email, password, etc.
    email: String,
    password: String,
    confirmed: bool,
    signed_up: DateTime<Utc>,
    confirmation_token: String,
}

// Implement methods for the User struct
impl User {
    // Implement methods related to user data
    pub fn new(email: String, password: String) -> Self {
        User {
            email,
            password,
            confirmed: false,
            signed_up: Utc::now(),
            confirmation_token: Self::generate_confirmation_token(),
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
// Models module


// Define the DatabasePath struct
pub struct DatabasePath {
    // Add fields such as user_id, database_name, path, etc.
}

// Implement methods for the DatabasePath struct
impl DatabasePath {
    // Implement methods related to database paths
}

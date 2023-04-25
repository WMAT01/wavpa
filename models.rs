use chrono::prelude::*;
use rand::Rng;
use uuid::Uuid;
use crate::database::{Database, SQLite3}; // Assuming SQLite3 as the database, update as needed

// Additional imports
use serde::{Serialize, Deserialize};

// In models.rs
use std::collections::HashMap;
// ...

use chrono::NaiveDateTime;
// ...
use bcrypt::{hash, verify, DEFAULT_COST};
const MAX_COST: u32 = 31;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AuthProvider {
    Local,
    Google,
    Microsoft,
    Facebook,
    Twitter,
    Apple,
}



// Define the User struct
pub struct User {
    id: Uuid,
    email: String,
//    password: String,
    pub password_hash: String,
    provider: AuthProvider,
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

impl From<Preferences> for HashMap<String, String> {
    fn from(p: Preferences) -> Self {
        let mut map = HashMap::new();
        map.insert("bool_1".to_string(), p.bool_1.to_string());
        map.insert("bool_2".to_string(), p.bool_2.to_string());
        map.insert("bool_3".to_string(), p.bool_3.to_string());
        map.insert("float_1".to_string(), p.float_1.to_string());
        map.insert("float_2".to_string(), p.float_2.to_string());
        map.insert("float_3".to_string(), p.float_3.to_string());
        map.insert("string_1".to_string(), p.string_1);
        map.insert("string_2".to_string(), p.string_2);
        map.insert("string_3".to_string(), p.string_3);
        map
    }
}


// Define the Session struct
/**
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
**/
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
//        let password_hash = hash(password, DEFAULT_COST)?;
        let password_hash = match provider {
            AuthProvider::Local => Some(hash(password, MAX_COST)?),
            _ => None,
        };

        User {
            id: Uuid::new_v4(),
            email,
//            password,
            password_hash,
            provider,
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

    pub fn create(email: &str, password: &str, name: &str) -> Result<Self, Error> {
//        let password_hash = hash(password, DEFAULT_COST)?;
        let password_hash = match provider {
            AuthProvider::Local => Some(hash(password, MAX_COST)?),
            _ => None,
        };
        // Save the new user with the hashed password
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }
}

// Additional imports
//use serde::{Serialize, Deserialize};

// ...

// Define the BillingDetails struct
#[derive(Serialize, Deserialize, Clone)]
pub struct BillingDetails {
    id: Uuid,
    payment_type: PaymentType,
    first_name: String,
    last_name: String,
    address1: String,
    address2: Option<String>,
    city: String,
    state: String,
    zip: String,
    amount: f32,
    first_successful_transaction: DateTime<Utc>,
    billing_duration: BillingDuration,
    transaction_attempts: Vec<Transaction>,
}

impl From<BillingDetails> for HashMap<String, String> {
    fn from(b: BillingDetails) -> Self {
        let mut map = HashMap::new();
        map.insert("uuid".to_string(), b.uuid.to_string());
        map.insert("payment_type".to_string(), format!("{:?}", b.payment_type));
        map.insert("first_name".to_string(), b.first_name);
        map.insert("last_name".to_string(), b.last_name);
        map.insert("address1".to_string(), b.address1);
        map.insert("address2".to_string(), b.address2);
        map.insert("city".to_string(), b.city);
        map.insert("state".to_string(), b.state);
        map.insert("zip".to_string(), b.zip);
        map.insert("amount".to_string(), b.amount.to_string());
        map.insert("first_successful_transaction".to_string(), b.first_successful_transaction.to_string());
        map.insert("billing_duration".to_string(), format!("{:?}", b.billing_duration));
        // Add transaction attempts
        let transaction_attempts: Vec<String> = b.transaction_attempts.iter().map(|attempt| attempt.uuid.to_string()).collect();
        map.insert("transaction_attempts".to_string(), transaction_attempts.join(", "));
        map
    }
}



// Define the PaymentType enum
#[derive(Serialize, Deserialize, Clone)]
pub enum PaymentType {
    GooglePay,
    ApplePay,
    PayPal,
    CashApp,
    Venmo,
    Zelle,
}

// Define the BillingDuration enum
#[derive(Serialize, Deserialize, Clone)]
pub enum BillingDuration {
    Monthly,
    Quarterly,
    Yearly,
}

// Define the Transaction struct
#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    id: Uuid,
    billing_details_id: Uuid,
    user_id: Uuid,
    started_transaction: DateTime<Utc>,
    finished_transaction: DateTime<Utc>,
    result: TransactionResult,
}

// Define the TransactionResult enum
#[derive(Serialize, Deserialize, Clone)]
pub enum TransactionResult {
    Success,
    Failure,
    Unknown,
    Error(String),
}

impl From<Preferences> for HashMap<String, String> {
    fn from(pref: Preferences) -> Self {
        let mut map = HashMap::new();
        map.insert("database_type".to_string(), pref.database_type);
        // Add other preferences fields here
        map
    }
}

impl From<BillingDetails> for HashMap<String, String> {
    fn from(details: BillingDetails) -> Self {
        let mut map = HashMap::new();
        map.insert("payment_type".to_string(), details.payment_type);
        // Add other billing details fields here
        map
    }
}

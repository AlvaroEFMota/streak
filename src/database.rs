use chrono::{NaiveDateTime, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Deserializer};
use std::sync::{Arc, Mutex};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

lazy_static::lazy_static! {
    static ref DATABASE: Arc<Mutex<Database>> = Arc::new(Mutex::new(Database::new()));
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub email: String,
}

#[derive(Debug)]
pub struct Activity {
    pub id: Option<i64>,
    pub user_email: String,
    pub name: String,
    pub accumulative: i64,
    pub streak: i64,
    pub last_update: NaiveDateTime,
}

impl Activity {
    fn new(user_email: String, name: String) -> Self {
        Activity {
            id: None,
            user_email,
            name,
            accumulative: 0,
            streak: 0,
            last_update: Utc::now().naive_utc(),
        }
    }
}

impl<'de> Deserialize<'de> for Activity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ActivityDeserialize {
            user_email: String,
            name: String,
        }

        let activity_deserealize = ActivityDeserialize::deserialize(deserializer)?;
        Ok(Activity::new(activity_deserealize.user_email, activity_deserealize.name))
    }
}

pub struct Database {
    pub connection: Connection,
}

impl Database {
    fn new() -> Self {
        let connection = Connection::open("database.db").expect("Could not open the database");
        Self { connection }
    }

    pub fn create_database(&self) {
        self.connection
            .execute("PRAGMA foreign_keys = ON", ())
            .unwrap();
        self.connection
            .execute(
                "CREATE TABLE IF NOT EXISTS user (
            email TEXT PRIMARY KEY
        )",
                (),
            )
            .unwrap();
        self.connection
            .execute(
                "CREATE TABLE IF NOT EXISTS activity (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_email TEXT NOT NULL,
            name TEXT NOT NULL,
            accumulative INTEGER NOT NULL,
            streak INTEGER NOT NULL,
            last_update TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_email) REFERENCES user(email)
            UNIQUE (user_email, name)
        )",
                (), // empty list of parameters.
            )
            .unwrap();
    }

    pub fn insert_user(&self, user: &User) -> Result<()> {
        self.connection
            .execute("INSERT INTO user (email) VALUES (?1)", [user.email.clone()])?;
        Ok(())
    }

    pub fn insert_activity(&self, activity: &Activity) -> Result<()> {
        self.connection.execute("INSERT INTO activity (user_email, name, accumulative, streak, last_update) VALUES (?1, ?2, ?3, ?4, ?5)", [activity.user_email.clone(), activity.name.clone(), activity.accumulative.to_string(), activity.streak.to_string(), activity.last_update.to_string()])?;
        Ok(())
    }
}

pub fn get_database() -> Arc<Mutex<Database>> {
    DATABASE.clone()
}

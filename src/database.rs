//use actix_web::body::BoxBod
//use actix_web::Responder;
use chrono::{Duration, NaiveDateTime, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

lazy_static::lazy_static! {
    static ref DATABASE: Arc<Mutex<Database>> = Arc::new(Mutex::new(Database::new()));
}

#[derive(Deserialize, Debug)]
pub struct UserCreate {
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct Activity {
    pub id: u64,
    pub user_email: String,
    pub name: String,
    pub accumulative: u64,
    pub streak: u64,
    pub last_update: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct ActivityCreate {
    pub user_email: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct ActivityUpdate {
    pub id: u64,
    pub duration: u64,
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

    pub fn insert_user(&self, user: &UserCreate) -> Result<()> {
        self.connection
            .execute("INSERT INTO user (email) VALUES (?1)", [user.email.clone()])?;
        Ok(())
    }

    pub fn insert_activity(&self, activity_create: &ActivityCreate) -> Result<()> {
        let last_update = Utc::now().naive_utc().timestamp().to_string();

        self.connection.execute(
            "INSERT INTO activity (user_email, name, accumulative, streak, last_update) VALUES (?1, ?2, ?3, ?4, ?5)",
            [activity_create.user_email.clone(),
            activity_create.name.clone(),
            "0".to_string(),
            "0".to_string(),
            last_update])?;
        Ok(())
    }

    pub fn get_activity(&self, id: u64) -> std::result::Result<Activity, rusqlite::Error> {
        let activity = self.connection.query_row(
            "SELECT id, user_email, name, accumulative, streak, last_update FROM activity WHERE id = ?",
            [id],
            |row| {
                let last_update_timestamp = row.get(5)?;
                let last_update = NaiveDateTime::from_timestamp_opt(last_update_timestamp, 0).unwrap();

                Ok(Activity {
                id: row.get(0)?,
                    user_email: row.get(1)?,
                    name: row.get(2)?,
                    accumulative: row.get(3)?,
                    streak: row.get(4)?,
                    last_update,
                })
            });
        println!("{:?}", activity);
        activity
        //Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Custom Error")))
    }

    pub fn update_activity(&self, activity_update: &ActivityUpdate) -> Result<()> {
        let activity = self.get_activity(activity_update.id)?;
        println!("activity: {:?}", activity);
        println!("activity_update{:?}", activity_update);
        let mut streak: u64 = activity.streak + 1;
        if activity.last_update + Duration::days(2) < Utc::now().naive_utc() {
            streak = 1;
        }
        let last_update = Utc::now().naive_utc().timestamp().to_string();

        let accumulative = activity.accumulative + activity_update.duration;
        println!("accumulative {}", accumulative);

        self.connection.execute(
            "UPDATE activity SET accumulative = ?, streak = ?, last_update = ? WHERE id = ?",
            [
                accumulative.to_string(),
                streak.to_string(),
                last_update,
                activity_update.id.to_string(),
            ],
        )?;
        Ok(())
    }
}

pub fn get_database() -> Arc<Mutex<Database>> {
    DATABASE.clone()
}

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

pub mod garmin;
pub mod schema;

use crate::garmin::{Activity, InsertableActivity};
use diesel::{Connection, PgConnection, QueryResult, RunQueryDsl};
use dotenv::dotenv;
use schema::activities;
use std::env;
use std::error::Error;

pub fn read_csv(path: &str) -> Result<Vec<garmin::InsertableActivity>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_path(path)?;

    let records: Result<Vec<InsertableActivity>, _> = reader.deserialize().into_iter().collect();
    Ok(records?)
}

pub fn all_activities(connection: &PgConnection) -> QueryResult<Vec<Activity>> {
    activities::table.load::<Activity>(&*connection)
}

pub fn insert_activity(
    activity: InsertableActivity,
    connection: &PgConnection,
) -> QueryResult<Activity> {
    diesel::insert_into(activities::table)
        .values(&activity)
        .get_result(connection)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

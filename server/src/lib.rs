extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::collections::HashMap;
use std::env;
use std::error::Error;

use diesel::dsl::sum;
use diesel::query_dsl::QueryDsl;
use diesel::{Connection, PgConnection, QueryResult, RunQueryDsl};
use dotenv::dotenv;

use crate::db::{Activity, ActivityType, InsertableActivity};
use crate::schema::activities;
use crate::schema::activity_type;

pub mod db;
pub mod garmin;
pub mod schema;

pub fn all_activities(connection: &PgConnection) -> QueryResult<Vec<Activity>> {
    activities::table.load::<Activity>(connection)
}

pub fn total_km(connection: &PgConnection) -> QueryResult<Option<f64>> {
    activities::table
        .inner_join(activity_type::table)
        .select(sum(activities::distance * activity_type::scale))
        .first::<Option<f64>>(connection)
}

pub fn get_activity_map(connection: &PgConnection) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    let x = activity_type::table.load::<ActivityType>(connection)?;
    Ok(x.into_iter().map(|x| x.as_name_to_id()).collect())
}

pub fn insert_activities(
    activity_vec: Vec<InsertableActivity>,
    connection: &PgConnection,
) -> QueryResult<usize> {
    diesel::insert_into(activities::table)
        .values(activity_vec)
        .on_conflict(activities::date)
        .do_nothing()
        .execute(connection)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

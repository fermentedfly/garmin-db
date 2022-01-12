extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::collections::HashMap;
use std::env;
use std::error::Error;

use diesel::dsl::sum;
use diesel::prelude::*;
use dotenv::dotenv;

use crate::db::{Activity, ActivityType, NewActivity, NewUser, User};
use crate::schema::activities;
use crate::schema::activity_type;
use crate::schema::users;

pub mod db;
pub mod garmin;
pub mod schema;

pub fn all_activities(connection: &PgConnection) -> QueryResult<Vec<Activity>> {
    activities::table.load::<Activity>(connection)
}

pub fn total_km(
    connection: &PgConnection,
    user_name: &str,
    include_elevation: bool,
) -> QueryResult<Option<f64>> {
    let scaled_distance = activities::distance * activity_type::scale;
    let scaled_elevation = activities::elevation * activity_type::elevation_scale;

    let q = activities::table
        .inner_join(activity_type::table)
        .inner_join(users::table)
        .filter(users::user_name.eq(user_name));

    if include_elevation {
        q.select(sum(scaled_distance + scaled_elevation))
            .first(connection)
    } else {
        q.select(sum(scaled_distance)).first(connection)
    }
}

pub fn get_activity_map(connection: &PgConnection) -> Result<HashMap<String, i32>, Box<dyn Error>> {
    let x = activity_type::table.load::<ActivityType>(connection)?;
    Ok(x.into_iter().map(|x| x.as_name_to_id()).collect())
}

pub fn insert_activities(
    activity_vec: Vec<NewActivity>,
    connection: &PgConnection,
) -> QueryResult<usize> {
    diesel::insert_into(activities::table)
        .values(activity_vec)
        .on_conflict(activities::date)
        .do_nothing()
        .execute(connection)
}

pub fn add_user(name: &String, connection: &PgConnection) -> QueryResult<User> {
    let new_user = NewUser::new(name);
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(connection)
}

pub fn get_user_by_name(name: &str, connection: &PgConnection) -> QueryResult<User> {
    users::table
        .filter(users::user_name.eq(name))
        .select(users::all_columns)
        .get_result(connection)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

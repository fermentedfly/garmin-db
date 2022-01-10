extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::collections::HashMap;
use std::env;
use std::error::Error;

use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::{Connection, PgConnection, QueryResult, RunQueryDsl};
use dotenv::dotenv;

use crate::garmin::{Activity, ActivityType, CSVActivity, InsertableActivity};
use crate::schema::activities;
use crate::schema::activity_type;

pub mod garmin;
pub mod schema;

pub fn read_csv(path: &str) -> Result<Vec<garmin::CSVActivity>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_path(path)?;

    let records: Result<Vec<CSVActivity>, _> = reader.deserialize().into_iter().collect();
    Ok(records?)
}

pub fn all_activities(connection: &PgConnection) -> QueryResult<Vec<Activity>> {
    activities::table.load::<Activity>(connection)
}

pub fn insert_activity(activity: CSVActivity, connection: &PgConnection) -> QueryResult<usize> {
    let activity_type: i32 = activity_type::table
        .select(activity_type::id)
        .filter(activity_type::name.eq(&activity.activity_type))
        .first(connection)?;

    let new_activity = InsertableActivity::new(
        activity.title,
        activity_type,
        activity.date,
        activity.time,
        activity.distance,
        activity.elevation,
    );

    let query = diesel::insert_into(activities::table).values(new_activity);
    query.execute(connection)
}

pub fn insert_activities(
    activity_vec: Vec<CSVActivity>,
    connection: &PgConnection,
) -> Result<usize, Box<dyn Error>> {
    connection.transaction(|| {
        let x = activity_type::table.load::<ActivityType>(connection)?;
        let m: HashMap<String, i32> = x.into_iter().map(|x| x.as_name_to_id()).collect();

        let to_insert: Result<Vec<_>, _> = activity_vec
            .into_iter()
            .map(|x| match m.get(&x.activity_type) {
                Some(type_id) => Ok(InsertableActivity::new(
                    x.title,
                    *type_id,
                    x.date,
                    x.time,
                    x.distance,
                    x.elevation,
                )),
                None => Err("Bad activity type"),
            })
            .collect();
        let to_insert = to_insert?;

        let res = diesel::insert_into(activities::table)
            .values(to_insert)
            .on_conflict(activities::date)
            .do_nothing()
            .execute(connection);
        Ok(res?)
    })
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

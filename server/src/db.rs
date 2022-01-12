use crate::schema::{activities, activity_type, users};

use chrono::NaiveDateTime;
use diesel::data_types::PgInterval;
use diesel::dsl::sum;
use diesel::prelude::*;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::error::Error;

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "activity_type"]
pub struct ActivityType {
    pub id: i32,
    pub name: String,
    pub scale: f64,
    pub elevation_scale: f64,
}

impl ActivityType {
    pub fn as_name_to_id(&self) -> (String, i32) {
        (self.name.clone(), self.id)
    }
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub user_name: String,
}

#[derive(Insertable, Clone)]
#[table_name = "users"]
pub struct NewUser {
    user_name: String,
}

impl NewUser {
    pub fn new(name: &String) -> NewUser {
        NewUser {
            user_name: name.clone(),
        }
    }
}

#[derive(Identifiable, Associations, Queryable, Debug, Clone, PartialEq)]
#[table_name = "activities"]
#[belongs_to(ActivityType)]
#[belongs_to(User)]
pub struct Activity {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
    pub activity_type_id: i32,
    pub date: NaiveDateTime,
    pub time: PgInterval,
    pub distance: f64,
    pub elevation: f64,
}

#[derive(Associations, Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "activities"]
pub struct NewActivity {
    title: String,
    user_id: i32,
    activity_type_id: i32,
    date: NaiveDateTime,
    time: PgInterval,
    distance: f64,
    elevation: f64,
}

impl NewActivity {
    pub fn new(
        title: &String,
        user_id: i32,
        activity_type_id: i32,
        date: NaiveDateTime,
        time: PgInterval,
        distance: f64,
        elevation: f64,
    ) -> NewActivity {
        NewActivity {
            title: title.clone(),
            user_id,
            activity_type_id,
            date,
            time,
            distance,
            elevation,
        }
    }
}

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

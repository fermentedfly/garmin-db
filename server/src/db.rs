use crate::schema::{activities, activity_type, users};

use chrono::NaiveDateTime;
use diesel::data_types::PgInterval;

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

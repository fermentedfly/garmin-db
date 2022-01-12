use crate::schema::{activities, activity_type};

use chrono::NaiveDateTime;
use diesel::data_types::PgInterval;

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "activity_type"]
pub struct ActivityType {
    id: i32,
    name: String,
    scale: f64,
    elevation_scale: f64,
}

impl ActivityType {
    pub fn as_name_to_id(&self) -> (String, i32) {
        (self.name.clone(), self.id)
    }
}

#[derive(Identifiable, Associations, Queryable, Debug, Clone, PartialEq)]
#[table_name = "activities"]
#[belongs_to(ActivityType)]
pub struct Activity {
    id: i32,
    title: String,
    activity_type_id: i32,
    date: NaiveDateTime,
    time: PgInterval,
    distance: f64,
    elevation: f64,
}

#[derive(Associations, Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "activities"]
#[belongs_to(ActivityType)]
pub struct InsertableActivity {
    title: String,
    activity_type_id: i32,
    date: NaiveDateTime,
    time: PgInterval,
    distance: f64,
    elevation: f64,
}

impl InsertableActivity {
    pub fn new(
        title: &String,
        activity_type_id: i32,
        date: NaiveDateTime,
        time: PgInterval,
        distance: f64,
        elevation: f64,
    ) -> InsertableActivity {
        InsertableActivity {
            title: title.clone(),
            activity_type_id,
            date,
            time,
            distance,
            elevation,
        }
    }
}

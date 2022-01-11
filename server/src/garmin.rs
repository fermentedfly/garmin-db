use crate::schema::{activities, activity_type};

use chrono::{NaiveDateTime, NaiveTime, Timelike};
use diesel::data_types::PgInterval;
use serde::{de, Deserialize, Deserializer};

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "activity_type"]
pub struct ActivityType {
    id: i32,
    name: String,
    scale: f64,
    elevation_scale: Option<f64>,
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
        title: String,
        activity_type_id: i32,
        date: NaiveDateTime,
        time: PgInterval,
        distance: f64,
        elevation: f64,
    ) -> InsertableActivity {
        InsertableActivity {
            title,
            activity_type_id,
            date,
            time,
            distance,
            elevation,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct CSVActivity {
    pub(crate) title: String,
    #[serde(rename(deserialize = "Activity Type"))]
    pub(crate) activity_type: String,
    #[serde(deserialize_with = "de_naive_date_time")]
    pub(crate) date: NaiveDateTime,
    #[serde(deserialize_with = "de_pg_interval")]
    pub(crate) time: PgInterval,
    #[serde(deserialize_with = "de_f64")]
    pub(crate) distance: f64,
    #[serde(rename(deserialize = "Total Ascent"))]
    #[serde(deserialize_with = "de_f64")]
    pub(crate) elevation: f64,
}

fn de_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s == "--" {
        return Ok(0.0);
    }
    let val: f64 = s.trim().replace(",", "").parse().unwrap();
    Ok(val)
}

fn de_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(de::Error::custom)
    // TODO fix timezone
}

fn de_pg_interval<'de, D>(deserializer: D) -> Result<PgInterval, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let s = s.trim();
    let pattern = if s.contains(".") {
        "%H:%M:%S%.f"
    } else {
        "%H:%M:%S"
    };
    let time = NaiveTime::parse_from_str(&s, pattern).map_err(de::Error::custom)?;
    let u_seconds = ((time.hour() * 60 + time.minute()) * 60 + time.second()) * 1000
        + (time.nanosecond() / 1000);
    Ok(PgInterval::from_microseconds(u_seconds as i64))
}

use crate::schema::activities;

use chrono::{NaiveDateTime, NaiveTime, Timelike};
use diesel::data_types::PgInterval;
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "activities"]
pub struct Activity {
    id: i32,
    title: String,
    activity_type: String,
    date: NaiveDateTime,
    time: PgInterval,
    distance: f64,
    elevation: f64,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "activities"]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct InsertableActivity {
    title: String,
    #[serde(rename(deserialize = "Activity Type"))]
    activity_type: String,
    #[serde(deserialize_with = "de_naive_date_time")]
    date: NaiveDateTime,
    #[serde(deserialize_with = "de_pg_interval")]
    time: PgInterval,
    #[serde(deserialize_with = "de_f64")]
    distance: f64,
    #[serde(rename(deserialize = "Elev Gain"))]
    #[serde(deserialize_with = "de_f64")]
    elevation: f64,
}

impl InsertableActivity {
    fn from_activity(activity: Activity) -> InsertableActivity {
        InsertableActivity {
            title: activity.title,
            activity_type: activity.activity_type,
            date: activity.date,
            time: activity.time,
            distance: activity.distance,
            elevation: activity.elevation,
        }
    }
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

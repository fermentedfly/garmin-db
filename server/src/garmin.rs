use crate::schema::activities;

use chrono::{NaiveDateTime, NaiveTime};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Queryable, Insertable, Deserialize, Debug)]
#[table_name = "activities"]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Activity {
    title: String,
    #[serde(rename(deserialize = "Activity Type"))]
    activity_type: String,
    #[serde(deserialize_with = "de_naive_date_time")]
    date: NaiveDateTime,
    #[serde(deserialize_with = "de_naive_time")]
    time: NaiveTime,
    #[serde(deserialize_with = "de_f64")]
    distance: f64,
    #[serde(rename(deserialize = "Elev Gain"))]
    #[serde(deserialize_with = "de_f64")]
    elevation: f64,
}

impl fmt::Display for Activity
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}: {}, {}km, {}m {}", self.title, self.date, self.activity_type, self.distance, self.elevation, self.time)
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

fn de_naive_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
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
    NaiveTime::parse_from_str(&s, pattern).map_err(de::Error::custom)
}

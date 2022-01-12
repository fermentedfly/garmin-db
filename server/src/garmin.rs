use chrono::{NaiveDateTime, NaiveTime, Timelike};
use diesel::data_types::PgInterval;
use serde::{de, Deserialize, Deserializer};
use std::collections::HashMap;
use std::error::Error;

use crate::NewActivity;

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct CSVActivity {
    title: String,
    #[serde(rename(deserialize = "Activity Type"))]
    activity_type: String,
    #[serde(deserialize_with = "de_naive_date_time")]
    date: NaiveDateTime,
    #[serde(deserialize_with = "de_pg_interval")]
    time: PgInterval,
    #[serde(deserialize_with = "de_f64")]
    distance: f64,
    #[serde(rename(deserialize = "Total Ascent"))]
    #[serde(deserialize_with = "de_f64")]
    elevation: f64,
}

impl CSVActivity {
    fn to_insertable_activity(
        &self,
        activity_id_map: &HashMap<String, i32>,
        user_id: i32,
    ) -> Result<NewActivity, &'static str> {
        // convert garmin activity type
        let activity_name = match self.activity_type.as_str() {
            "Open Water Swimming" => "Swimming",
            "Climbing" => "Mountaineering",
            s => s,
        };

        // fix swimming distance given as meters
        let distance = match activity_name {
            "Swimming" => self.distance * 1e-3,
            _ => self.distance,
        };

        if let Some(activity_type_id) = activity_id_map.get(activity_name as &str) {
            Ok(NewActivity::new(
                &self.title,
                user_id,
                *activity_type_id,
                self.date,
                self.time,
                distance,
                self.elevation,
            ))
        } else {
            Err("bad activity name")
        }
    }
}

pub fn read_csv(
    path: &str,
    activity_id_map: &HashMap<String, i32>,
    user_id: i32,
) -> Result<Vec<NewActivity>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_path(path)?;

    let records: Vec<_> = reader
        .deserialize::<CSVActivity>()
        .into_iter()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.to_insertable_activity(activity_id_map, user_id).ok())
        .collect();
    Ok(records)
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

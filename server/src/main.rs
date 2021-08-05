use chrono::{NaiveDateTime, NaiveTime};
use std::error::Error;
use std::io;
use std::process;

use serde::{de, Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
enum ActivityType {
    Cycling,
    Swimming,
    Running,
    Hiking,
    Climbing,
    Walking,
    Rowing,
    #[serde(rename = "Mountain Biking")]
    MountainBiking,
    Mountaineering,
    Snowshoeing,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct Activity {
    title: String,
    #[serde(rename(deserialize = "Activity Type"))]
    activity_type: ActivityType,
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

//"2021-08-04 05:40:58"

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Activity = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

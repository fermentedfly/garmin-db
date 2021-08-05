use std::error::Error;
use std::io;
use std::process;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize= "PascalCase"))]
struct Activity {
    title: String,
    #[serde(rename(deserialize = "Activity Type"))]
    activity_type: String,
    date: String,
    time: String,
    #[serde(deserialize_with = "de_f64")]
    distance: f64,
    #[serde(rename(deserialize = "Elev Gain"))]
    #[serde(deserialize_with = "de_f64")]
    elevation: f64
}

fn de_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    if s == "--"
    {
        return Ok(0.0)
    }
    let val : f64 = s.trim().replace(",", "").parse().unwrap();
    Ok(val)
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().delimiter(b',').from_reader(io::stdin());
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
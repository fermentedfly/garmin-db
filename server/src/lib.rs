#[macro_use]
extern crate diesel;
extern crate chrono;

pub mod garmin;
pub mod schema;

use self::garmin::Activity;

use std::error::Error;

pub fn read_csv(path: &str) -> Result<Vec<garmin::Activity>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_path(path)?;

    let records: Result<Vec<Activity>, _> = reader.deserialize().into_iter().collect();
    Ok(records?)
}

#[cfg(test)]
mod tests {
    use super::*;
}
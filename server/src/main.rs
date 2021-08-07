pub mod garmin;

use std::env;
use std::error::Error;
use std::process;

fn read_csv(path: &str) -> Result<Vec<garmin::Activity>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_path(path)?;

    let records: Result<Vec<garmin::Activity>, _> = reader.deserialize().into_iter().collect();
    Ok(records?)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("error missing csv file name");
        process::exit(1);
    }

    let filename = &args[1];

    match read_csv(filename) {
        Ok(data) => {
            for d in data {
                println!("{:?}", d);
            }
        }
        Err(err) => {
            println!("error decoding csv: {}", err);
        }
    }
}

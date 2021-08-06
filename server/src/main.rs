pub mod garmin;

use std::error::Error;
use std::io;
use std::process;

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: garmin::Activity = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

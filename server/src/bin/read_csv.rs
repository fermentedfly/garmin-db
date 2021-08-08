use std::{env, process};
use garmin_db_server::read_csv;

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
                println!("{}", d);
            }
        }
        Err(err) => {
            println!("error decoding csv: {}", err);
        }
    }
}

use garmin_db_server::{establish_connection, insert_activities, read_csv};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("error missing csv file name");
        process::exit(1);
    }

    let filename = &args[1];

    let connection = establish_connection();

    match read_csv(filename) {
        Ok(data) => match insert_activities(data, &connection) {
            Ok(nr_lines) => {
                println!("inserted {} activities", nr_lines);
            }
            Err(err) => {
                println!("error during insert: {}", err);
            }
        },
        Err(err) => {
            println!("error decoding csv: {}", err);
        }
    }
}

use garmin_db_server::garmin::read_csv;
use garmin_db_server::{establish_connection, get_activity_map};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("error missing csv file name");
        process::exit(1);
    }

    let filename = &args[1];

    let connection = establish_connection();

    let am = get_activity_map(&connection).unwrap();

    match read_csv(filename, &am) {
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

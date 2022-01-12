use diesel::Connection;
use garmin_db_server::garmin::read_csv;
use garmin_db_server::{establish_connection, get_activity_map, insert_activities};
use std::error::Error;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("error missing csv file name");
        process::exit(1);
    }

    let filename = &args[1];

    let connection = establish_connection();

    match connection.transaction::<_, Box<dyn Error>, _>(|| {
        let am = get_activity_map(&connection).unwrap();
        let data = read_csv(filename, &am)?;
        let nr_lines = insert_activities(data, &connection)?;
        Ok(nr_lines)
    }) {
        Ok(nr_lines) => println!("inserted {} activities", nr_lines),
        Err(e) => println!("error: {}", e),
    }
}

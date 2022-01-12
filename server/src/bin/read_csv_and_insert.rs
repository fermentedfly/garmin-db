use clap::{App, Arg};
use diesel::Connection;
use garmin_db_server::garmin::read_csv;
use garmin_db_server::{
    establish_connection, get_activity_map, get_user_by_name, insert_activities,
};
use std::error::Error;

fn main() {
    let matches = App::new("Insert Garmin CSV into DB")
        .arg(
            Arg::new("filename")
                .short('f')
                .long("filename")
                .takes_value(true)
                .help("CSV file to be inserted")
                .required(true),
        )
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .takes_value(true)
                .help("name of user data is to be inserted for")
                .required(true),
        )
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let username = matches.value_of("username").unwrap();

    let connection = establish_connection();

    match connection.transaction::<_, Box<dyn Error>, _>(|| {
        let user = get_user_by_name(username, &connection)?;
        let am = get_activity_map(&connection).unwrap();
        let data = read_csv(filename, &am, user.id)?;
        let nr_lines = insert_activities(data, &connection)?;
        Ok(nr_lines)
    }) {
        Ok(nr_lines) => println!("inserted {} activities", nr_lines),
        Err(e) => println!("error: {}", e),
    }
}

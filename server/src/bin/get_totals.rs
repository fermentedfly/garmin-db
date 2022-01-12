use clap::{App, Arg};
use garmin_db_server::{establish_connection, total_km};

fn main() {
    let matches = App::new("Get totals of a user")
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .takes_value(true)
                .help("name of user data is to be inserted for")
                .required(true),
        )
        .get_matches();

    let user_name = matches.value_of("username").unwrap();

    let connection = establish_connection();
    match total_km(&connection, user_name, true) {
        Ok(data) => {
            println!("Overall km: {}", data.unwrap());
        }
        Err(err) => {
            println!("error reading from database: {}", err);
        }
    }
}

use garmin_db_server::db::{add_user, establish_connection};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("error missing user name");
        process::exit(1);
    }
    let connection = establish_connection();
    let user_name = &args[1];
    match add_user(user_name, &connection) {
        Ok(u) => println!("Added user: [{}] -> {}", u.id, u.user_name),
        Err(_) => println!("Error during add user"),
    }
}

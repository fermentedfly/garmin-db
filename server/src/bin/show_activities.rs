use garmin_db_server::db::{all_activities, establish_connection};

fn main() {
    let connection = establish_connection();
    let res = all_activities(&connection);

    match res {
        Ok(data) => {
            for d in data {
                println!("{:?}", d);
            }
        }
        Err(err) => {
            println!("error reading from database: {}", err);
        }
    }
}

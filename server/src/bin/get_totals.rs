use garmin_db_server::{establish_connection, total_km};

fn main() {
    let connection = establish_connection();
    match total_km(&connection) {
        Ok(data) => {
            println!("Overall km: {}", data.unwrap());
        }
        Err(err) => {
            println!("error reading from database: {}", err);
        }
    }
}

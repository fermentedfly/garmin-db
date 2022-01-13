use crate::db::total_km;
use crate::DBPool;
use rocket;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/overall/<user_name>")]
pub fn get_overall(conn: DBPool, user_name: String) -> Result<Json<Option<f64>>, Status> {
    total_km(&*conn, &user_name, true)
        .map(|x| Json(x))
        .map_err(|_| Status::InternalServerError)
}

#[get("/overall/<user_name>/<year>")]
pub fn get_overall_by_year(conn: DBPool, user_name: String, year: u32) -> String {
    format!("{}", 0)
}

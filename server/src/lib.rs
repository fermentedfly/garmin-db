#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use diesel::PgConnection;

mod schema;

pub mod db;
pub mod garmin;
pub mod routes;

use rocket_contrib::databases::database;

#[database("garmin_db")]
pub struct DBPool(PgConnection);

pub fn launch() {
    rocket::ignite()
        .mount(
            "/",
            routes![routes::get_overall, routes::get_overall_by_year],
        )
        .attach(DBPool::fairing())
        .launch();
}

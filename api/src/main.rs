// #![feature(proc_macro_hygiene, decl_macro)]
//
// #[macro_use] extern crate rocket;
//
// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }
//
// fn main() {
//     rocket::ignite().mount("/", routes![index]).launch();
// }

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;

mod controllers;
mod db;
mod jwt;
mod models;
mod schema;
mod utils;

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/user", controllers::user::routes())
        .mount("/car", controllers::car::routes())
        .mount("/bank", controllers::bank::routes())
        .register(utils::catcher::catchers())
        .launch();
}
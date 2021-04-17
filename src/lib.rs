#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use dotenv::dotenv;

mod config;
mod routes;
mod db;
mod models;
mod errors;

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::ignite()
        .mount(
            "/api",
            routes![
                routes::books::get_books,
                routes::books::get_book,
                routes::books::create_book
            ],
        )
        .manage(config::init())
}
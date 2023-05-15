#[macro_use]
extern crate rocket;

mod catcher;
mod types;
mod db;
mod state;
mod article;
mod auth;
mod config;

use article::route::{index, route_article};
use config::get_custom_figment;
use db::{init_db_blog};
use state::{get_default_user_token, init_app_state};
use catcher::{ error_catcher, not_found_catcher, bad_request_catcher };

#[launch]
fn rocket() -> _ {
    rocket::custom(get_custom_figment())
        .attach(init_db_blog())
        .attach(get_default_user_token())
        .manage(init_app_state())
        .register("/", catchers![error_catcher, not_found_catcher, bad_request_catcher])
        .mount("/", routes![index, route_article])
}

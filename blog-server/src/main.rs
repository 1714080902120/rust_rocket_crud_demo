#[macro_use]
extern crate rocket;

mod article;
mod auth;
mod catcher;
mod config;
mod db;
mod state;
mod types;

use article::route::{index, route_article, get_user_article};
use auth::route::{login, register};
use catcher::{bad_request_catcher, error_catcher, not_found_catcher};
use config::{get_custom_figment, init_my_config};
use db::init_db_blog;
use state::{get_default_user_token, init_validate_instace};

#[launch]
fn rocket() -> _ {
    rocket::custom(get_custom_figment())
        .attach(init_my_config())
        .attach(init_db_blog())
        .attach(get_default_user_token())
        .manage(init_validate_instace().unwrap())
        .register(
            "/",
            catchers![error_catcher, not_found_catcher, bad_request_catcher],
        )
        .mount("/", routes![index, route_article])
        .mount("/user", routes![login, register, get_user_article])
}

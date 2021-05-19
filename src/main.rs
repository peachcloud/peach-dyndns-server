#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::routes::{index, register_domain, check_available};
use rocket::figment::{Figment, providers::{Format, Toml, Env}};

mod cli;
mod routes;
mod errors;
mod constants;
mod generate_zone;

#[tokio::main]
async fn main() {
    let _args = cli::args().expect("error parsing args");

    // the following config says to use all default rocket configs
    // and then override them with any configs specified in Rocket.toml if found
    // and then override with any configs specified as env variables prefixed with APP_
    let config = Figment::from(rocket::Config::default())
      .merge(Toml::file("Rocket.toml").nested()).merge(Env::prefixed("ROCKET_").global());

    let rocket_result = rocket::custom(config)
        .mount("/", routes![index, register_domain, check_available])
        .launch()
        .await;

    if let Err(err) = rocket_result {
        error!("++ error launching rocket server: {:?}", err);
    }
}

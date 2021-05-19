#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::routes::{index, register_domain, check_available};
use rocket::Config;
use rocket::figment::{Figment, Profile, providers::{Format, Toml, Serialized, Env}};

mod cli;
mod routes;
mod errors;
mod constants;
mod generate_zone;

#[tokio::main]
async fn main() {
    let _args = cli::args().expect("error parsing args");

    // the following config says to use all default rocket configs
    // and then override them with any configs specified in Rocket.toml
    let config = Figment::from(rocket::Config::default())
      .merge(Toml::file("Rocket.toml").nested());

    let rocket_result = rocket::custom(config)
        .mount("/", routes![index, register_domain, check_available])
        .launch()
        .await;

    if let Err(err) = rocket_result {
        error!("++ error launching rocket server: {:?}", err);
    }
}

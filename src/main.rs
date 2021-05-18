#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::routes::{index, register_domain};
use futures::try_join;
use std::io;
use tokio::task;

mod cli;
mod routes;
mod errors;
mod constants;
mod generate_zone;

#[tokio::main]
async fn main() {
    let rocket_result = rocket::build()
        .mount("/", routes![index, register_domain])
        .launch()
        .await;

    if let Err(err) = rocket_result {
        error!("++ error launching rocket server: {:?}", err);
    }
}

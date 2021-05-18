#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::routes::{index, register_domain, check_available};

mod cli;
mod routes;
mod errors;
mod constants;
mod generate_zone;

#[tokio::main]
async fn main() {
    let _args = cli::args().expect("error parsing args");

    let rocket_result = rocket::build()
        .mount("/", routes![index, register_domain, check_available])
        .launch()
        .await;

    if let Err(err) = rocket_result {
        error!("++ error launching rocket server: {:?}", err);
    }
}

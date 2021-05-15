#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use futures::try_join;
use std::io;
use tokio::task;
use crate::http::{index, register_domain};

mod cli;
mod http;
mod client;

#[tokio::main]
async fn main() {

     let rocket_result= rocket::build()
        .mount("/", routes![index, register_domain])
        .launch()
        .await;

    if let Err(err) = rocket_result {
        error!("++ error launching rocket server: {:?}", err);
    }

}

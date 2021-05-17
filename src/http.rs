/*
*
* /register-user (sends an email verification to create a new account)
* /verify (for clicking the link in the email)
* /register-domain (add a new domain and get back the secret for subsequent updating)
* /update-domain (update the IP for the domain, passing the associated secret)
*
*/
use crate::generate_zone::{check_domain_available, generate_zone};
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use std::thread;

#[get("/")]
pub fn index() -> &'static str {
    "This is the peach-dyn-dns server."
}

#[derive(Deserialize, Debug)]
pub struct RegisterDomainPost {
    domain: String,
}

#[post("/register-domain", data = "<data>")]
pub async fn register_domain(data: Json<RegisterDomainPost>) -> &'static str {
    info!("++ post request to register new domain: {:?}", data);
    // TODO: first confirm domain is in the right format ("*.dyn.peachcloud.org")
    let is_domain_available = check_domain_available(&data.domain);
    if !is_domain_available{
        "can't register domain that already exists"
    } else {
        let result = generate_zone(&data.domain);
        match result {
            Ok(key_file_text) => {
                // TODO: figure out how to return key_file_text
                "successfully created zone"
            }
            Err(err) => {
                "there was an error registering the domain"
            }
        }
    }
}

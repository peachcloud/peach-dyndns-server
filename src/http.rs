/*
*
* /register-user (sends an email verification to create a new account)
* /verify (for clicking the link in the email)
* /register-domain (add a new domain and get back the secret for subsequent updating)
* /update-domain (update the IP for the domain, passing the associated secret)
*
*/
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::thread;
use crate::client::check_domain_available;

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
    let handle = thread::spawn(move || {
        let domain_already_exists = check_domain_available(&data.domain);
        domain_already_exists
    });
    let domain_already_exists = handle.join().unwrap();
    if domain_already_exists {
        "can't register domain already exists"
    } else {
        // TODO: use bash to generate a tsig key, update bind config, and then return the secret
        "New domain registered"
    }
}
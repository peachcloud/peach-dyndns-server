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

#[get("/")]
fn index() -> &'static str {
    "This is the peach-dyn-dns server."
}

#[derive(Deserialize, Debug)]
struct RegisterDomainPost {
    domain: String,
}

#[post("/register-domain", data = "<data>")]
fn register_domain(data: Json<RegisterDomainPost>) -> &'static str {
    info!("++ post request to register new domain: {:?}", data);
    "New domain registered" // TODO: return secret
}

#[derive(Deserialize, Debug)]
struct UpdateDomainPost {
    domain: String,
    secret: String,
}

#[post("/update-domain", data = "<data>")]
fn update_domain(data: Json<UpdateDomainPost>) -> &'static str {
    info!("++ post request to update domain: {:?}", data);
    "Updating domain" // TODO: validate, then do it
}

pub async fn server() {

    let rocket_result= rocket::build()
        .mount("/", routes![index, register_domain, update_domain])
        .launch()
        .await;

    if let Err(err) = rocket_result {
        error!("++ error launching rocket server: {:?}", err);
    }
}

/*
*
* /register-user (sends an email verification to create a new account)
* /verify (for clicking the link in the email)
* /register-domain (add a new domain and get back the secret for subsequent updating)
* /update-domain (update the IP for the domain, passing the associated secret)
*
*/
use rocket_contrib::json::Json;
use rocket::State;
use rocket::{http::Status, response::Responder};

use sled_extensions::bincode::Tree;
use sled_extensions::DbExt;
use serde::{Deserialize, Serialize};
use std::error::Error;

use std::{error, io};
use serde_json::Error as SerdeError;
use snafu::{Snafu, ResultExt};
use std::option::NoneError;

pub type BoxError = Box<dyn error::Error>;

#[derive(Debug, Snafu)]
pub enum
ServerError {
    #[snafu(display("This is a test error: {}", msg))]
    TestError { msg: String },
    #[snafu(display("Sled Error: {}", source))]
    SledError { source: sled_extensions::Error },
    #[snafu(display("The secret associated with this domain did not match"))]
    SecretMismatch { domain: String },
    NotFound,
}

impl From<NoneError> for ServerError {
    fn from(_: NoneError) -> Self {
        ServerError::NotFound
    }
}

impl<'a, 'b: 'a> Responder<'a, 'static> for ServerError {
    fn respond_to(self, _: &rocket::Request) -> Result<rocket::Response<'static>, Status> {
        match self {
            TestError => {
                Err(Status::InternalServerError)
            },
            SecretMismatch=> Err(Status::Forbidden)
        }
    }
}

type EndpointResult<T> = Result<T, ServerError>;



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

#[get("/test_error")]
fn test_error() -> Result<&'static str, ServerError> {
    info!("++ get request to test_error");
    return TestError { msg: format!("this is a test error") }.fail();
    Ok("test error request")
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct DomainRecord {
    domain: String,
    secret: String,
    ip: String
}


#[post("/domain/update", data = "<data>")]
fn update_domain(db: State<Database>, data: Json<DomainRecord>) -> EndpointResult<Json<DomainRecord>> {
    info!("++ post request to update domain: {:?}", data);
    // first check if a domain record with this domain name already exists
    let previous_domain_record = db.domains.get(data.domain.as_bytes()).context(SledError)?;
    info!("++ found previous_domain_record: {:?}", previous_domain_record);
    // then match based on Some Or None
    match previous_domain_record {
        // if a record already exists with this domain name
        Some(previous) => {
            // check if the secret matches up
            if data.secret == previous.secret {
                info!("++ secrets match");
                // if the secret matches, then update the record
                db.domains
                    .insert(data.domain.as_bytes(), data.clone())
                    .unwrap();
            // if the secret doesn't match, then return an error
            } else {
                info!("++ secrets mismatch");
                // TODO: this isn't properly throwing an error
                return SecretMismatch { domain: format!("{:?}", data.domain) }.fail();
            }
        },
        // if a previous_domain_record was not found with this domain name, then create one
        None => {
            db.domains
                .insert(data.domain.as_bytes(), data.clone())
                .unwrap();
          }
        }
    Ok(Json(data.0))
}


struct Database {
    domains: Tree<DomainRecord>,
}

pub async fn server() {

    let db = sled_extensions::Config::default()
        .path("./sled_data")
        .open()
        .expect("Failed to open sled db");

    let rocket_result= rocket::build()
        .manage(Database {
            domains: db.open_bincode_tree("domains").expect("could not create sled tree"),
         })
        .mount("/", routes![index, register_domain, update_domain, test_error])
        .launch()
        .await;

    if let Err(err) = rocket_result {
        error!("++ error launching rocket server: {:?}", err);
    }
}

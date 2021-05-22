/*
* LIST OF ROUTES
* /domain/register (add a new domain and get back the TSIG key for subsequent updating with nsupdate)
* /domain/check-available (check if given domain is available)
* /user/register sends an email verification to create a new account) NOT IMPLEMENTED
* /user/verify (for clicking the link in the email) NOT IMPLEMENTED
*/
mod errors;
mod generate_zone;
mod constants;
use crate::generate_zone::{check_domain_available, generate_zone, validate_domain};
use std::result::Result;
use jsonrpc_core::{types::error::Error, IoHandler, Params, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use log::info;
use std::env;


use crate::errors::{BoxError, PeachDynDnsError};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct RegisterDomainPost {
    domain: String,
}

/// Create JSON-RPC I/O handler, add RPC methods and launch HTTP server.
pub fn run() -> Result<(), BoxError> {
    info!("Starting up.");

    info!("Creating JSON-RPC I/O handler.");
    let mut io = IoHandler::new();

    io.add_method("ping", |_| Ok(Value::String("success".to_string())));

    io.add_method("register_domain", move |params: Params| {
        let d: Result<RegisterDomainPost, Error> = params.parse();
        match d {
            Ok(d) => match generate_zone(&d.domain) {
                Ok(_) => Ok(Value::String("success".to_string())),
                Err(e) => Err(Error::from(e)),
            },
            Err(e) => Err(Error::from(PeachDynDnsError::MissingParams { e })),
        }
    });

    let http_server =
        env::var("PEACH_DYNDNS_SERVER").unwrap_or_else(|_| "127.0.0.1:3001".to_string());

    info!("Starting JSON-RPC server on {}.", http_server);
    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(
            &http_server
                .parse()
                .expect("Invalid HTTP address and port combination"),
        )
        .expect("Unable to start RPC server");

    server.wait();

    Ok(())
}



//
//#[get("/")]
//pub fn index() -> &'static str {
//    "This is the peach-dyndns server."
//}
//
//#[derive(Serialize)]
//pub struct JsonResponse {
//    pub status: String,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    pub data: Option<JsonValue>,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    pub msg: Option<String>,
//}
//
//// helper function to build a JsonResponse object
//pub fn build_json_response(
//    status: String,
//    data: Option<JsonValue>,
//    msg: Option<String>,
//) -> JsonResponse {
//    JsonResponse { status, data, msg }
//}
//
//#[derive(Deserialize, Debug)]
//pub struct RegisterDomainPost {
//    domain: String,
//}
//
//#[post("/domain/register", data = "<data>")]
//pub async fn register_domain(data: Json<RegisterDomainPost>) -> Json<JsonResponse> {
//    info!("++ post request to register new domain: {:?}", data);
//    // TODO: grab/create a mutex, so that only one rocket thread is calling register_domain at a time
//    // check if its a valid domain
//    if !validate_domain(&data.domain) {
//        let status = "error".to_string();
//        let msg = "domain is not in a valid format".to_string();
//        Json(build_json_response(status, None, Some(msg)))
//    } else {
//        // check if the domain is available
//        let is_domain_available = check_domain_available(&data.domain);
//        if !is_domain_available {
//            let status = "error".to_string();
//            let msg = "can't register a domain that is already registered".to_string();
//            Json(build_json_response(status, None, Some(msg)))
//        } else {
//            // generate configs for the zone
//            let result = generate_zone(&data.domain);
//            match result {
//                Ok(key_file_text) => {
//                    let status = "success".to_string();
//                    let msg = key_file_text.to_string();
//                    Json(build_json_response(status, None, Some(msg)))
//                }
//                Err(_err) => {
//                    let status = "error".to_string();
//                    let msg = "there was an error creating the zone file".to_string();
//                    Json(build_json_response(status, None, Some(msg)))
//                }
//            }
//        }
//    }
//}
//
//#[derive(Deserialize, Debug)]
//pub struct CheckAvailableDomainPost {
//    domain: String,
//}
//
//// route which returns a msg of "true" if the domain is available and "false" if it is already taken
//#[post("/domain/check-available", data = "<data>")]
//pub async fn check_available(data: Json<CheckAvailableDomainPost>) -> Json<JsonResponse> {
//    info!("post request to check if domain is available {:?}", data);
//     if !validate_domain(&data.domain) {
//        let status = "error".to_string();
//        let msg = "domain is not in a valid format".to_string();
//        Json(build_json_response(status, None, Some(msg)))
//    } else {
//         let status = "success".to_string();
//         let is_available = check_domain_available(&data.domain);
//         let msg = is_available.to_string();
//         Json(build_json_response(status, None, Some(msg)))
//     }
//}

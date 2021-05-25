/*
* LIST OF METHODS
* register_domain (add a new domain and get back the TSIG key for subsequent updating with nsupdate)
* is_domain_available (check if given domain is available and return boolean result)
* register_user (sends an email verification to create a new account) NOT IMPLEMENTED
* verify_user (for clicking the link in the email) NOT IMPLEMENTED
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

#[derive(Deserialize, Debug)]
pub struct IsDomainAvailablePost {
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
                // returns full TSIG key text to new zone as part of success result
                Ok(key_text) => Ok(Value::String(key_text)),
                Err(e) => Err(Error::from(e)),
            },
            Err(e) => Err(Error::from(PeachDynDnsError::MissingParams { e })),
        }
    });

    io.add_method("is_domain_available", move |params: Params| {
        let d: Result<IsDomainAvailablePost, Error> = params.parse();
        match d {
            Ok(d) => {
                // if the domain has an invalid format return an erro
                if !validate_domain(&d.domain) {
                    Err(Error::from(PeachDynDnsError::InvalidDomain{ domain: d.domain.to_string() }))
                }
                // if it has a valid format, check if its available
                else {
                    let result = check_domain_available(&d.domain);
                    Ok(Value::Bool(result))
                }
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

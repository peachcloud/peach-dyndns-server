use std::{error, str};

use jsonrpc_core::{types::error::Error, ErrorCode};
use snafu::Snafu;
use std::string::FromUtf8Error;

pub type BoxError = Box<dyn error::Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum PeachDynDnsError {
    #[snafu(display("Missing expected parameters: {}", e))]
    MissingParams { e: Error },

    #[snafu(display("Domain is in invalid format: {}", domain))]
    InvalidDomain { domain: String },

    #[snafu(display("There was an error in the bind configuration"))]
    BindConfigurationError,

    #[snafu(display("This domain was already registered: {}", domain))]
    DomainAlreadyExistsError { domain: String },

    #[snafu(display("Error parsing key file: {}", source))]
    KeyFileParseError { source: FromUtf8Error },

    #[snafu(display("Error generating key: {}", source))]
    KeyGenerationError { source: std::io::Error },
}

impl From<PeachDynDnsError> for Error {
    fn from(err: PeachDynDnsError) -> Self {
        match &err {
            PeachDynDnsError::MissingParams { e } => e.clone(),
            PeachDynDnsError::InvalidDomain { domain } => Error {
                code: ErrorCode::ServerError(-32028),
                message: format!("Domain is invalid format: {}", domain),
                data: None,
            },
            PeachDynDnsError::BindConfigurationError => Error {
                code: ErrorCode::ServerError(-32029),
                message: "There was a bind configuration error".to_string(),
                data: None,
            },
            PeachDynDnsError::DomainAlreadyExistsError { domain } => Error {
                code: ErrorCode::ServerError(-32030),
                message: format!("Can't register domain that already exists: {}", domain),
                data: None,
            },
            PeachDynDnsError::KeyFileParseError { source: _ } => Error {
                code: ErrorCode::ServerError(-32031),
                message: "Error parsing key file".to_string(),
                data: None,
            },
            PeachDynDnsError::KeyGenerationError { source: _ } => Error {
                code: ErrorCode::ServerError(-32032),
                message: "Key generation error".to_string(),
                data: None,
            },
        }
    }
}

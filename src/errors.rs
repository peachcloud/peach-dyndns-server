use std::string::FromUtf8Error;


#[derive(Debug)]
pub enum PeachDynError {
    GenerateTsigIoError(std::io::Error),
    GenerateTsigParseError(std::string::FromUtf8Error),
    DomainAlreadyExistsError(String),
    BindConfigurationError(String),
}

impl From<std::io::Error> for PeachDynError {
    fn from(err: std::io::Error) -> PeachDynError {
        PeachDynError::GenerateTsigIoError(err)
    }
}

impl From<FromUtf8Error> for PeachDynError {
    fn from(err: std::string::FromUtf8Error) -> PeachDynError {
        PeachDynError::GenerateTsigParseError(err)
    }
}
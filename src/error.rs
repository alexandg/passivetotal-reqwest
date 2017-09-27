use std::error;
use std::fmt;
use std::result;

use reqwest;

/// `passivetotal-reqest` specific `Result` type.
///
/// Returned by all queries to indicate a variety of possible errors from
/// problems with HTTP request to parsing errors.
pub type Result<T> = result::Result<T, PassiveTotalError>;

/// Represents various errors that can occur during API requests.
#[derive(Debug)]
pub enum PassiveTotalError {
    /// Returned when no valid domain can be parsed from a given query
    InvalidDomain,
    /// An error returned by `reqwest`.
    ReqwestError(reqwest::Error),
    /// Requests to the Passivetotal API that return a status code 400-499
    ///
    /// This represents errors that occur with queries sent to the Passivetotal
    /// API by the Client.
    ClientError(reqwest::StatusCode),
    /// Requests to the Passivetotal API that return a status code 500-599
    ///
    /// This represents errors that occur on the server of the Passivetotal API.
    ServerError(reqwest::StatusCode),
    /// An error when creating an `SslField` enum from a `&str`
    SslFieldParseError(String),
    /// An error when creating an `WhoisField` enum from a `&str`
    WhoisFieldParseError(String),
}

impl fmt::Display for PassiveTotalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PassiveTotalError::InvalidDomain => write!(f, "Invalid domain given."),
            PassiveTotalError::ReqwestError(ref err) => write!(f, "Reqwest error: {}", err),
            PassiveTotalError::ClientError(ref code) => write!(f, "Client error: {}", code),
            PassiveTotalError::ServerError(ref code) => write!(f, "Server error: {}", code),
            PassiveTotalError::SslFieldParseError(ref s) => {
                write!(f, "Error parsing SslField from {}", s)
            },
            PassiveTotalError::WhoisFieldParseError(ref s) => {
                write!(f, "Error parsing WhoisField from {}", s)
            },
        }
    }
}

impl error::Error for PassiveTotalError {
    fn description(&self) -> &str {
        match *self {
            PassiveTotalError::InvalidDomain => "Invalid domain given.",
            PassiveTotalError::ReqwestError(ref err) => error::Error::description(err),
            PassiveTotalError::ClientError(_) => {
                "Client error. This is usually caused by a malformed request."
            },
            PassiveTotalError::ServerError(_) => {
                "Server error. This is usually caused by an error on the PassiveTotal server."
            },
            PassiveTotalError::SslFieldParseError(_) => {
                "SslFieldParse error. Unable to parse given string into an SslField."
            },
            PassiveTotalError::WhoisFieldParseError(_) => {
                "WhoisFieldParse error. Unable to parse given string into an WhoisField."
            },
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PassiveTotalError::ReqwestError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for PassiveTotalError {
    fn from(err: reqwest::Error) -> PassiveTotalError {
        PassiveTotalError::ReqwestError(err)
    }
}

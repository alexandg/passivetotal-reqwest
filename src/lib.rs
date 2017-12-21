//! Passivetotal [`reqwest`][1]-based API
//!
//! A [`reqwest`][1] based version of the [`passivetotal-rs`][2] crate. Used for
//! querying the [Passivetotal][3] v2 [API][4].
//!
//! Requires a valid Passivetotal username and apikey.
//!
//! # Usage
//!
//! To use `passivetotal-reqwest` add it to your `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! passivetotal-reqwest = { git =
//! "https://github.com/alexandg/passivetotal-reqwest" }
//! ```
//!
//! Then add it to your crate:
//!
//! ```
//! extern crate passivetotal_reqwest
//! ```
//!
//! All methods that make Passivetotal API requests return a
//! `Result<serde_json::Value>` to allow access to the JSON object returned
//! in Passivetotal API responses.
//!
//! # Example
//!
//! ```no_run
//! use passivetotal_reqwest::PassiveTotal;
//!
//! let pt = PassiveTotal::with_auth("username", "apikey");
//! let resp = pt.passive_dns("www.passivetotal.org").unwrap();
//!
//! println!("{}", resp)
//! ```
//!
//! # License
//!
//! `passivetotal-reqwest` is licensed under the MIT License.
//!
//! [1]: https://crates.io/crates/reqwest
//! [2]: https://github.com/passivetotal/rust_api
//! [3]: https://www.passivetotal.org
//! [4]: https://api.passivetotal.org/api/docs/
extern crate chrono;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate url;

use std::str::FromStr;
use std::time::Duration;

use serde_json::Value;

#[macro_use]
mod macros;
mod account;
mod actions;
mod enrichment;
mod error;
mod ssl;
mod utils;

pub use error::{PassiveTotalError, Result};

const BASE_URL: &str = "https://api.passivetotal.org/v2";

/// Represents the available WHOIS search fields for WHOIS field searches
#[derive(Debug)]
pub enum WhoisField {
    Email,
    Domain,
    Name,
    Organization,
    Address,
    Phone,
    Nameserver,
}

impl WhoisField {
    /// Returns a `&str` representation of an `WhoisField` enum
    pub fn as_str(&self) -> &str {
        match *self {
            WhoisField::Email => "email",
            WhoisField::Domain => "domain",
            WhoisField::Name => "name",
            WhoisField::Organization => "organization",
            WhoisField::Address => "address",
            WhoisField::Phone => "phone",
            WhoisField::Nameserver => "nameserver",
        }
    }
}

impl FromStr for WhoisField {
    type Err = PassiveTotalError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "email" => Ok(WhoisField::Email),
            "domain" => Ok(WhoisField::Domain),
            "name" => Ok(WhoisField::Name),
            "organization" => Ok(WhoisField::Organization),
            "address" => Ok(WhoisField::Address),
            "phone" => Ok(WhoisField::Phone),
            "nameserver" => Ok(WhoisField::Nameserver),
            _ => Err(PassiveTotalError::WhoisFieldParseError(String::from(s))),
        }
    }
}

/// Struct used to access the Passivetotal v2 API.
#[derive(Debug)]
pub struct PassiveTotal {
    username: String,
    apikey: String,
    timeout: Duration,
}

impl PassiveTotal {
    /// Create a new PassiveTotal struct.
    pub fn new<S>(username: S, apikey: S, timeout: Duration) -> Self
    where
        S: Into<String>,
    {
        PassiveTotal {
            username: username.into(),
            apikey: apikey.into(),
            timeout: timeout,
        }
    }

    /// Create a new PassiveTotal struct using the given username and API key.
    ///
    /// Use this to create a new PassiveTotal struct with the given username
    /// and API key and the default timeout of 60 seconds.
    pub fn with_auth<S>(username: S, apikey: S) -> Self
    where
        S: Into<String>,
    {
        PassiveTotal {
            username: username.into(),
            apikey: apikey.into(),
            timeout: Duration::from_secs(60),
        }
    }

    /// Lookup the passive DNS information for a given domain or ip address.
    pub fn passive_dns(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/dns/passive",
            json!({
                "query": utils::valid_domain(query)?
            }),
        )
    }

    /// Lookup unique passive DNS information for a given domain or ip address.
    pub fn unique_passive_dns(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/dns/passive/unique",
            json!({
                "query": utils::valid_domain(query)?
            }),
        )
    }

    /// Retrieve WHOIS information for a given domain.
    pub fn whois(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/whois",
            json!({
                "query": utils::valid_domain(query)?
            }),
        )
    }

    /// Search WHOIS data for a keyword.
    pub fn whois_search_keyword(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/whois/search/keyword",
            json!({
                "query": query
            }),
        )
    }

    /// Searches WHOIS data by field and query.
    pub fn whois_search(&self, query: &str, field: WhoisField) -> Result<Value> {
        self.send_request_json_response(
            "/whois/search",
            json!({
                "query": query,
                "field": field.as_str()
            }),
        )
    }

    fn send_request_json_response<T>(&self, url: &str, params: T) -> Result<Value>
    where
        T: serde::ser::Serialize,
    {
        let url = format!("{}{}", BASE_URL, url);
        let mut resp = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()?
            .get(&url)
            .basic_auth(self.username.as_str(), Some(self.apikey.as_str()))
            .json(&params)
            .send()?;

        if resp.status().is_client_error() {
            Err(PassiveTotalError::ClientError(resp.status()))
        } else if resp.status().is_server_error() {
            Err(PassiveTotalError::ServerError(resp.status()))
        } else {
            resp.json().map_err(From::from)
        }
    }
}

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
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate url;

use std::time::Duration;

use serde_json::Value;

#[macro_use]
mod macros;
mod account;
mod actions;
mod enrichment;
mod error;
mod ssl;
mod passive;
mod utils;
mod whois;

pub use error::{PassiveTotalError, Result};

const BASE_URL: &str = "https://api.passivetotal.org/v2";

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
            timeout,
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

    fn send_request_json_response<T>(&self, endpoint: &str, params: T) -> Result<Value>
    where
        T: serde::ser::Serialize,
    {
        let url = format!("{}{}", BASE_URL, endpoint);
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

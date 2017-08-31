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
//! `passivetotal-reqwest` is licensed under the MIT License. See LICENSE-MIT.
//!
//! [1]: https://crates.io/crates/reqwest
//! [2]: https://github.com/passivetotal/rust_api
//! [3]: https://www.passivetotal.org
//! [4]: https://api.passivetotal.org/api/docs/
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_json;

use std::str::FromStr;
use std::time::Duration;

use serde_json::Value;

mod error;

pub use error::{PassiveTotalError, Result};

const BASE_URL: &'static str = "https://api.passivetotal.org/v2";

/// Represents the available ssl search fields for ssl field searches
#[derive(Debug)]
pub enum SslField {
    IssuerSurname,
    SubjectOrganizationName,
    IssuerCountry,
    IssuerOrganizationUnitName,
    Fingerprint,
    SubjectOrganizationUnitName,
    SerialNumber,
    SubjectEmailAddress,
    SubjectCountry,
    IssuerGivenName,
    SubjectCommonName,
    IssuerCommonName,
    IssuerStateOrProvinceName,
    IssuerProvince,
    SubjectStateOrProvinceName,
    Sha1,
    SubjectStreetAddress,
    SubjectSerialNumber,
    IssuerOrganizationName,
    SubjectSurname,
    SubjectLocalityName,
    IssuerStreetAddress,
    IssuerLocalityName,
    SubjectGivenName,
    SubjectProvince,
    IssuerSerialNumber,
    IssuerEmailAddress,
}

impl SslField {
    /// Returns a `&str` representation of an `SslField` enum
    pub fn as_str(&self) -> &str {
        match *self {
            SslField::IssuerSurname => "issuerSurname",
            SslField::SubjectOrganizationName => "subjectOrganizationName",
            SslField::IssuerCountry => "issuerCountry",
            SslField::IssuerOrganizationUnitName => "issuerOrganizationUnitName",
            SslField::Fingerprint => "fingerprint",
            SslField::SubjectOrganizationUnitName => "subjectOrganizationUnitName",
            SslField::SerialNumber => "serialNumber",
            SslField::SubjectEmailAddress => "subjectEmailAddress",
            SslField::SubjectCountry => "subjectCountry",
            SslField::IssuerGivenName => "issuerGivenName",
            SslField::SubjectCommonName => "subjectCommonName",
            SslField::IssuerCommonName => "issuerCommonName",
            SslField::IssuerStateOrProvinceName => "issuerStateOrProvinceName",
            SslField::IssuerProvince => "issuerProvince",
            SslField::SubjectStateOrProvinceName => "subjectStateOrProvinceName",
            SslField::Sha1 => "sha1",
            SslField::SubjectStreetAddress => "subjectStreetAddress",
            SslField::SubjectSerialNumber => "subjectSerialNumber",
            SslField::IssuerOrganizationName => "issuerOrganizationName",
            SslField::SubjectSurname => "subjectSurname",
            SslField::SubjectLocalityName => "subjectLocalityName",
            SslField::IssuerStreetAddress => "issuerStreetAddress",
            SslField::IssuerLocalityName => "issuerLocalityName",
            SslField::SubjectGivenName => "subjectGivenName",
            SslField::SubjectProvince => "subjectProvince",
            SslField::IssuerSerialNumber => "issuerSerialNumber",
            SslField::IssuerEmailAddress => "issuerEmailAddress",
        }
    }
}

impl FromStr for SslField {
    type Err = PassiveTotalError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "issuersurname" => Ok(SslField::IssuerSurname),
            "subjectorganizationname" => Ok(SslField::SubjectOrganizationName),
            "issuercountry" => Ok(SslField::IssuerCountry),
            "issuerorganizationunitname" => Ok(SslField::IssuerOrganizationUnitName),
            "fingerprint" => Ok(SslField::Fingerprint),
            "subjectorganizationunitname" => Ok(SslField::SubjectOrganizationUnitName),
            "serialnumber" => Ok(SslField::SerialNumber),
            "subjectemailaddress" => Ok(SslField::SubjectEmailAddress),
            "subjectcountry" => Ok(SslField::SubjectCountry),
            "issuergivenname" => Ok(SslField::IssuerGivenName),
            "subjectcommonname" => Ok(SslField::SubjectCommonName),
            "issuercommonname" => Ok(SslField::IssuerCommonName),
            "issuerstateorprovincename" => Ok(SslField::IssuerStateOrProvinceName),
            "issuerprovince" => Ok(SslField::IssuerProvince),
            "subjectstateorprovincename" => Ok(SslField::SubjectStateOrProvinceName),
            "sha1" => Ok(SslField::Sha1),
            "subjectstreetaddress" => Ok(SslField::SubjectStreetAddress),
            "subjectserialnumber" => Ok(SslField::SubjectSerialNumber),
            "issuerorganizationname" => Ok(SslField::IssuerOrganizationName),
            "subjectsurname" => Ok(SslField::SubjectSurname),
            "subjectlocalityname" => Ok(SslField::SubjectLocalityName),
            "issuerstreetaddress" => Ok(SslField::IssuerStreetAddress),
            "issuerlocalityname" => Ok(SslField::IssuerLocalityName),
            "subjectgivenname" => Ok(SslField::SubjectGivenName),
            "subjectprovince" => Ok(SslField::SubjectProvince),
            "issuerserialnumber" => Ok(SslField::IssuerSerialNumber),
            "issueremailaddress" => Ok(SslField::IssuerEmailAddress),
            _ => Err(PassiveTotalError::SslFieldParseError(String::from(s))),
        }
    }
}

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
                "query": query
            }),
        )
    }

    /// Lookup unique passive DNS information for a given domain or ip address.
    pub fn unique_passive_dns(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/dns/passive/unique",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieve WHOIS information for a given domain.
    pub fn whois(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/whois",
            json!({
                "query": query
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

    /// Retrieve a SSL certificate by SHA1 hash.
    pub fn sslcert(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/ssl-certificate",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieve the history of a SSL certificate but SHA1 hash or IP address.
    pub fn sslcert_history(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/ssl-certificate/history",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieves SSL certificates for a given keyword.
    pub fn sslcert_search(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/ssl-certificate/search/keyword",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieves SSL certificates for a given field value.
    pub fn sslcert_search_by_field(&self, query: &str, field: SslField) -> Result<Value> {
        self.send_request_json_response(
            "/ssl-certificate/search",
            json!({
                "query": query,
                "field": field.as_str(),
            }),
        )
    }

    /// Retrieve all available enrichment data for a domain or IP address.
    pub fn enrichment_data(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/enrichment",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieve OSINT data for a given domain or IP address.
    pub fn osint(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/enrichment/osint",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieve malware hosting information on a given domain or IP address.
    pub fn malware(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/enrichment/malware",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieve subdomains for a given domain.
    pub fn subdomains(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/enrichment/subdomains",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieve classification status for a given domain.
    pub fn classification(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/actions/classification",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieve if a given domain has ever been compromised.
    pub fn compromised(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/actions/ever-compromised",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieve if a given domain's DNS records are updated via dynamic DNS.
    pub fn ddns(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/actions/dynamic-dns",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieve if a given domain is being monitored.
    pub fn monitor(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/actions/monitor",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieve if a given domain is a sinkhole.
    pub fn sinkhole(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/actions/sinkhole",
            json!({
                "query": query
            }),
        )
    }

    /// Retrieve tags for a given query.
    pub fn tags(&self, query: &str) -> Result<Value> {
        self.send_request_json_response(
            "/actions/tags",
            json!({
                "query": query
            }),
        )
    }

    /// Read current account metadata and settings.
    pub fn account_info(&self) -> Result<Value> {
        self.send_request_json_response("/account", json!({}))
    }

    /// Read API usage history of current account.
    pub fn account_history(&self) -> Result<Value> {
        self.send_request_json_response("/account/history", json!({}))
    }

    /// Get active monitors.
    pub fn account_monitors(&self) -> Result<Value> {
        self.send_request_json_response("/account/monitors", json!({}))
    }

    /// Get current organization metadata.
    pub fn account_organization(&self) -> Result<Value> {
        self.send_request_json_response("/account/organization", json!({}))
    }

    /// Read current account and organization quotas.
    pub fn account_quotas(&self) -> Result<Value> {
        self.send_request_json_response("/account/quota", json!({}))
    }

    /// Check specific source being used for queries.
    pub fn account_source(&self, source: &str) -> Result<Value> {
        self.send_request_json_response("/account/sources", json!({"source": source}))
    }

    /// Read team activity.
    pub fn account_teamstream(&self) -> Result<Value> {
        self.send_request_json_response("/account/organization/teamstream", json!({}))
    }

    fn send_request_json_response<T>(&self, url: &str, params: T) -> Result<Value>
    where
        T: serde::ser::Serialize,
    {
        let url = format!("{}{}", BASE_URL, url);
        let mut resp = reqwest::Client::builder()?
            .timeout(self.timeout)
            .build()?
            .get(&url)?
            .basic_auth(self.username.as_str(), Some(self.apikey.as_str()))
            .json(&params)?
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

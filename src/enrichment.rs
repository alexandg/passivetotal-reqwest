use {PassiveTotal, PassiveTotalError, Result};

use serde_json::Value;
use url::{Host, Url};

const URL_DATA: &str = "/enrichment";
const URL_OSINT: &str = "/enrichment/osint";
const URL_MALWARE: &str = "/enrichment/malware";
const URL_SUBDOMAINS: &str = "/enrichment/subdomains";

macro_rules! impl_send_enrichment {
    ($id:ident) => {
        impl<'a> $id<'a> {
            pub fn send(&self) -> Result<Value> {
                self.pt.send_request_json_response(
                    self.url,
                    json!({
                        "query": valid_domain(&self.query)?
                    }),
                )
            }
        }
    };
}

// Check to see if a given `Host` is either a `Domain` or an `Ipv4`
// and return it as a `String` or return an `Error`
fn valid_host(host: Host) -> Result<String> {
    match host {
        Host::Domain(s) => Ok(s.to_owned()),
        Host::Ipv4(ip) => Ok(format!("{}", ip)),
        Host::Ipv6(_) => Err(PassiveTotalError::InvalidDomain),
    }
}

// Try to parse a given query into a valid domain or ipv4 address
fn valid_domain(query: &str) -> Result<String> {
    if let Ok(url) = Url::parse(query) {
        if let Some(host) = url.host() {
            return valid_host(host.to_owned());
        }
    }

    if let Ok(host) = Host::parse(query) {
        return valid_host(host.to_owned());
    }

    Err(PassiveTotalError::InvalidDomain)
}

pub struct EnrichmentRequest<'a> {
    pt: &'a PassiveTotal,
}

pub struct EnrichmentData<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: String,
}

pub struct EnrichmentOsint<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: String,
}

pub struct EnrichmentMalware<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: String,
}

pub struct EnrichmentSubdomains<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: String,
}

impl<'a> EnrichmentRequest<'a> {
    pub fn data<S>(self, query: S) -> EnrichmentData<'a> where S: Into<String> {
        EnrichmentData {
            pt: self.pt,
            url: URL_DATA,
            query: query.into(),
        }
    }

    pub fn osint<S>(self, query: S) -> EnrichmentOsint<'a> where S: Into<String> {
        EnrichmentOsint {
            pt: self.pt,
            url: URL_OSINT,
            query: query.into(),
        }
    }

    pub fn malware<S>(self, query: S) -> EnrichmentMalware<'a> where S: Into<String> {
        EnrichmentMalware {
            pt: self.pt,
            url: URL_MALWARE,
            query: query.into(),
        }
    }

    pub fn subdomains<S>(self, query: S) -> EnrichmentSubdomains<'a> where S: Into<String> {
        EnrichmentSubdomains {
            pt: self.pt,
            url: URL_SUBDOMAINS,
            query: query.into(),
        }
    }
}

impl_send_enrichment!(EnrichmentData);
impl_send_enrichment!(EnrichmentOsint);
impl_send_enrichment!(EnrichmentMalware);
impl_send_enrichment!(EnrichmentSubdomains);

impl PassiveTotal {
    pub fn enrichment(&self) -> EnrichmentRequest {
        EnrichmentRequest { pt: self }
    }
}

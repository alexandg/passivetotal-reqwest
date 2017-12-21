use {PassiveTotalError, Result};

use url::{Host, Url};

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
pub fn valid_domain(query: &str) -> Result<String> {
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

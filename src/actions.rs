use {PassiveTotal, PassiveTotalError, Result};

use serde_json::Value;
use url::{Host, Url};

const URL_CLASSIFICATION: &str = "/actions/classification";
const URL_COMPROMISED: &str = "/actions/ever-compromised";
const URL_DDNS: &str = "/actions/dynamic-dns";
const URL_MONITOR: &str = "/actions/monitor";
const URL_SINKHOLE: &str = "/actions/sinkhole";
const URL_TAGS: &str = "/actions/tags";

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

pub struct ActionsRequest<'a> {
    pt: &'a PassiveTotal,
}

pub struct ActionsClassification<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: &'a str,
}

pub struct ActionsCompromised<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: &'a str,
}
pub struct ActionsDynamicDns<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: &'a str,
}
pub struct ActionsMonitor<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: &'a str,
}
pub struct ActionsSinkhole<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: &'a str,
}
pub struct ActionsTags<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: &'a str,
}

impl<'a> ActionsRequest<'a> {
    pub fn classification(self, query: &'a str) -> ActionsClassification<'a> {
        ActionsClassification {
            pt: self.pt,
            url: URL_CLASSIFICATION,
            query: query,
        }
    }

    pub fn compromised(self, query: &'a str) -> ActionsCompromised<'a> {
        ActionsCompromised {
            pt: self.pt,
            url: URL_COMPROMISED,
            query: query,
        }
    }

    pub fn dynamic_dns(self, query: &'a str) -> ActionsDynamicDns<'a> {
        ActionsDynamicDns {
            pt: self.pt,
            url: URL_DDNS,
            query: query,
        }
    }

    pub fn monitor(self, query: &'a str) -> ActionsMonitor<'a> {
        ActionsMonitor {
            pt: self.pt,
            url: URL_MONITOR,
            query: query,
        }
    }

    pub fn sinkhole(self, query: &'a str) -> ActionsSinkhole<'a> {
        ActionsSinkhole {
            pt: self.pt,
            url: URL_SINKHOLE,
            query: query,
        }
    }

    pub fn tags(self, query: &'a str) -> ActionsTags<'a> {
        ActionsTags {
            pt: self.pt,
            url: URL_TAGS,
            query: query,
        }
    }
}

impl_send_query!(ActionsClassification);
impl_send_query!(ActionsCompromised);
impl_send_query!(ActionsDynamicDns);
impl_send_query!(ActionsMonitor);
impl_send_query!(ActionsSinkhole);
impl_send_query!(ActionsTags);

impl PassiveTotal {
    pub fn actions(&self) -> ActionsRequest {
        ActionsRequest {
            pt: self,
        }
    }
}

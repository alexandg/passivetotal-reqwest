use {PassiveTotal, Result};

use serde_json::Value;

const URL_CLASSIFICATION: &str = "/actions/classification";
const URL_COMPROMISED: &str = "/actions/ever-compromised";
const URL_DDNS: &str = "/actions/dynamic-dns";
const URL_MONITOR: &str = "/actions/monitor";
const URL_SINKHOLE: &str = "/actions/sinkhole";
const URL_TAGS: &str = "/actions/tags";

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

impl_send_query_valid_domain!(ActionsClassification);
impl_send_query_valid_domain!(ActionsCompromised);
impl_send_query_valid_domain!(ActionsDynamicDns);
impl_send_query_valid_domain!(ActionsMonitor);
impl_send_query_valid_domain!(ActionsSinkhole);
impl_send_query_valid_domain!(ActionsTags);

impl PassiveTotal {
    pub fn actions(&self) -> ActionsRequest {
        ActionsRequest {
            pt: self,
        }
    }
}

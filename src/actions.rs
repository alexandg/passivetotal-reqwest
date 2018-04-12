use serde_json::Value;

use {PassiveTotal, Result};

const URL_CLASSIFICATION: &str = "/actions/classification";
const URL_COMPROMISED: &str = "/actions/ever-compromised";
const URL_DDNS: &str = "/actions/dynamic-dns";
const URL_MONITOR: &str = "/actions/monitor";
const URL_SINKHOLE: &str = "/actions/sinkhole";
const URL_TAGS: &str = "/actions/tags";

pub struct ActionsRequest<'a> {
    pt: &'a PassiveTotal,
}

request_struct!(ActionsClassification {
    query: &'a str,
});

request_struct!(ActionsCompromised {
    query: &'a str,
});

request_struct!(ActionsDynamicDns {
    query: &'a str,
});

request_struct!(ActionsMonitor {
    query: &'a str,
});

request_struct!(ActionsSinkhole {
    query: &'a str,
});

request_struct!(ActionsTags {
    query: &'a str
});

impl<'a> ActionsRequest<'a> {
    pub fn classification(self, query: &'a str) -> ActionsClassification<'a> {
        ActionsClassification {
            pt: self.pt,
            url: URL_CLASSIFICATION,
            query,
        }
    }

    pub fn compromised(self, query: &'a str) -> ActionsCompromised<'a> {
        ActionsCompromised {
            pt: self.pt,
            url: URL_COMPROMISED,
            query,
        }
    }

    pub fn dynamic_dns(self, query: &'a str) -> ActionsDynamicDns<'a> {
        ActionsDynamicDns {
            pt: self.pt,
            url: URL_DDNS,
            query,
        }
    }

    pub fn monitor(self, query: &'a str) -> ActionsMonitor<'a> {
        ActionsMonitor {
            pt: self.pt,
            url: URL_MONITOR,
            query,
        }
    }

    pub fn sinkhole(self, query: &'a str) -> ActionsSinkhole<'a> {
        ActionsSinkhole {
            pt: self.pt,
            url: URL_SINKHOLE,
            query,
        }
    }

    pub fn tags(self, query: &'a str) -> ActionsTags<'a> {
        ActionsTags {
            pt: self.pt,
            url: URL_TAGS,
            query,
        }
    }
}

impl_send!(ActionsClassification);
impl_send!(ActionsCompromised);
impl_send!(ActionsDynamicDns);
impl_send!(ActionsMonitor);
impl_send!(ActionsSinkhole);
impl_send!(ActionsTags);

impl PassiveTotal {
    pub fn actions(&self) -> ActionsRequest {
        ActionsRequest { pt: self }
    }
}

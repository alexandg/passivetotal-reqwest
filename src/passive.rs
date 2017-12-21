use {PassiveTotal, PassiveTotalError, Result};
use utils;

use serde_json::Value;
use url::{Host, Url};

const URL_PDNS: &str = "/dns/passive";
const URL_PDNS_UNIQUE: &str = "/dns/passive/unique";

pub struct PassiveDnsRequest<'a> {
    pt: &'a PassiveTotal,
    query: &'a str,
    unique: bool,
}

impl PassiveTotal {
    pub fn passive_dns<'a>(&'a self, query: &'a str) -> PassiveDnsRequest<'a> {
        PassiveDnsRequest {
            pt: self,
            query: query,
            unique: false,
        }
    }
}

impl<'a> PassiveDnsRequest<'a> {
    pub fn send(&self) -> Result<Value> {
        let url = if self.unique {
            URL_PDNS_UNIQUE
        } else {
            URL_PDNS
        };

        self.pt.send_request_json_response(
            url,
            json!({ "query": utils::valid_domain(self.query)? })
        )
    }

    pub fn unique(&'a mut self, unique: bool) -> &'a mut PassiveDnsRequest {
        self.unique = unique;
        self
    }
}

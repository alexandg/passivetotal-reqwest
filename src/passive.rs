use serde_json::Value;

use {PassiveTotal, Result};

const URL_PDNS: &str = "/dns/passive";
const URL_PDNS_UNIQUE: &str = "/dns/passive/unique";

request_struct!(PassiveDnsRequest {
    query: &'a str,
});

request_struct!(PassiveDnsRequestUnique {
    query: &'a str,
});

impl PassiveTotal {
    pub fn passive_dns<'a>(&'a self, query: &'a str) -> PassiveDnsRequest<'a> {
        PassiveDnsRequest {
            pt: self,
            url: URL_PDNS,
            query,
        }
    }

    pub fn passive_dns_unique<'a>(&'a self, query: &'a str) -> PassiveDnsRequestUnique<'a> {
        PassiveDnsRequestUnique {
            pt: self,
            url: URL_PDNS_UNIQUE,
            query,
        }
    }
}

impl_send!(PassiveDnsRequest);
impl_send!(PassiveDnsRequestUnique);

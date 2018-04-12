use serde_json::Value;

use {PassiveTotal, Result};

const URL_DATA: &str = "/enrichment";
const URL_OSINT: &str = "/enrichment/osint";
const URL_MALWARE: &str = "/enrichment/malware";
const URL_SUBDOMAINS: &str = "/enrichment/subdomains";

pub struct EnrichmentRequest<'a> {
    pt: &'a PassiveTotal,
}

request_struct!(EnrichmentData { query: String });
request_struct!(EnrichmentOsint { query: String });
request_struct!(EnrichmentMalware { query: String });
request_struct!(EnrichmentSubdomains { query: String });

impl<'a> EnrichmentRequest<'a> {
    pub fn data<S>(self, query: S) -> EnrichmentData<'a>
    where
        S: Into<String>,
    {
        EnrichmentData {
            pt: self.pt,
            url: URL_DATA,
            query: query.into(),
        }
    }

    pub fn osint<S>(self, query: S) -> EnrichmentOsint<'a>
    where
        S: Into<String>,
    {
        EnrichmentOsint {
            pt: self.pt,
            url: URL_OSINT,
            query: query.into(),
        }
    }

    pub fn malware<S>(self, query: S) -> EnrichmentMalware<'a>
    where
        S: Into<String>,
    {
        EnrichmentMalware {
            pt: self.pt,
            url: URL_MALWARE,
            query: query.into(),
        }
    }

    pub fn subdomains<S>(self, query: S) -> EnrichmentSubdomains<'a>
    where
        S: Into<String>,
    {
        EnrichmentSubdomains {
            pt: self.pt,
            url: URL_SUBDOMAINS,
            query: query.into(),
        }
    }
}

impl_send!(EnrichmentData);
impl_send!(EnrichmentOsint);
impl_send!(EnrichmentMalware);
impl_send!(EnrichmentSubdomains);

impl PassiveTotal {
    pub fn enrichment(&self) -> EnrichmentRequest {
        EnrichmentRequest { pt: self }
    }
}

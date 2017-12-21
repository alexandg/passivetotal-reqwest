use {PassiveTotal, Result};

use serde_json::Value;

const URL_DATA: &str = "/enrichment";
const URL_OSINT: &str = "/enrichment/osint";
const URL_MALWARE: &str = "/enrichment/malware";
const URL_SUBDOMAINS: &str = "/enrichment/subdomains";

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

impl_send_query!(EnrichmentData);
impl_send_query!(EnrichmentOsint);
impl_send_query!(EnrichmentMalware);
impl_send_query!(EnrichmentSubdomains);

impl PassiveTotal {
    pub fn enrichment(&self) -> EnrichmentRequest {
        EnrichmentRequest { pt: self }
    }
}

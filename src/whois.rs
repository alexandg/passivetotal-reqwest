use {PassiveTotal, PassiveTotalError, Result};

use serde_json::Value;

use std::collections::HashMap;
use std::str::FromStr;

const URL_WHOIS: &str = "/whois";
const URL_WHOIS_KEYWORD: &str = "/whois/search/keyword";
const URL_WHOIS_SEARCH: &str = "/whois/search";

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

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
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

pub struct WhoisRequest<'a> {
    pt: &'a PassiveTotal,
}

impl<'a> WhoisRequest<'a> {
    pub fn information(self, query: &'a str) -> WhoisInfo {
        WhoisInfo {
            pt: self.pt,
            url: URL_WHOIS,
            query: query,
        }
    }

    pub fn search(self, query: &'a str) -> WhoisSearch {
        WhoisSearch {
            pt: self.pt,
            query: query,
            field: None,
        }
    }
}

pub struct WhoisInfo<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: &'a str,
}

impl_send_query_valid_domain!(WhoisInfo);

pub struct WhoisSearch<'a> {
    pt: &'a PassiveTotal,
    query: &'a str,
    field: Option<WhoisField>,
}

impl<'a> WhoisSearch<'a> {
    pub fn field(&'a mut self, field: WhoisField) -> &mut WhoisSearch {
        self.field = Some(field);
        self
    }

    pub fn send(&self) -> Result<Value> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        let url: &str;

        params.insert("query", self.query);
        if let Some(ref field) = self.field {
            url = URL_WHOIS_SEARCH;
            params.insert("field", field.as_str());
        } else {
            url = URL_WHOIS_KEYWORD;
        }

        self.pt.send_request_json_response(url, params)
    }
}

impl PassiveTotal {
    pub fn whois(&self) -> WhoisRequest {
        WhoisRequest {
            pt: self
        }
    }
}

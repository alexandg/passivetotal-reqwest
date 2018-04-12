use std::str::FromStr;

use serde::{Serialize, Serializer};
use serde_json::Value;

use {PassiveTotal, PassiveTotalError, Result};

const URL_WHOIS: &str = "/whois";
const URL_WHOIS_KEYWORD: &str = "/whois/search/keyword";
const URL_WHOIS_SEARCH: &str = "/whois/search";

pub struct WhoisRequest<'a> {
    pt: &'a PassiveTotal,
}

request_struct!(WhoisInfo {
    query: &'a str,
});

request_struct!(WhoisSearchField {
    query: &'a str,
    field: WhoisField,
});

request_struct!(WhoisSearchKeyword {
    query: &'a str,
});

/// Represents the available WHOIS search fields for WHOIS field searches
#[derive(Clone, Debug)]
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

impl Serialize for WhoisField {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'a> WhoisRequest<'a> {
    pub fn information(self, query: &'a str) -> WhoisInfo {
        WhoisInfo {
            pt: self.pt,
            url: URL_WHOIS,
            query,
        }
    }

    pub fn search_field(self, query: &'a str, field: WhoisField) -> WhoisSearchField {
        WhoisSearchField {
            pt: self.pt,
            url: URL_WHOIS_SEARCH,
            query,
            field,
        }
    }

    pub fn search_keyword(self, query: &'a str) -> WhoisSearchKeyword {
        WhoisSearchKeyword {
            pt: self.pt,
            url: URL_WHOIS_KEYWORD,
            query,
        }
    }
}

impl_send!(WhoisInfo);
impl_send!(WhoisSearchField);
impl_send!(WhoisSearchKeyword);

impl PassiveTotal {
    pub fn whois(&self) -> WhoisRequest {
        WhoisRequest { pt: self }
    }
}

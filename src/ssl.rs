use {PassiveTotal, PassiveTotalError, Result};

use serde_json::Value;

use std::collections::HashMap;
use std::str::FromStr;

const URL_SSL: &str = "/ssl-certificate";
const URL_HISTORY: &str = "/ssl-certificate/history";
const URL_KEYWORD: &str = "/ssl-certificate/search/keyword";
const URL_SEARCH: &str = "/ssl-certificate/search";

/// Represents the available ssl search fields for ssl field searches
#[derive(Debug)]
pub enum SslField {
    IssuerSurname,
    SubjectOrganizationName,
    IssuerCountry,
    IssuerOrganizationUnitName,
    Fingerprint,
    SubjectOrganizationUnitName,
    SerialNumber,
    SubjectEmailAddress,
    SubjectCountry,
    IssuerGivenName,
    SubjectCommonName,
    IssuerCommonName,
    IssuerStateOrProvinceName,
    IssuerProvince,
    SubjectStateOrProvinceName,
    Sha1,
    SubjectStreetAddress,
    SubjectSerialNumber,
    IssuerOrganizationName,
    SubjectSurname,
    SubjectLocalityName,
    IssuerStreetAddress,
    IssuerLocalityName,
    SubjectGivenName,
    SubjectProvince,
    IssuerSerialNumber,
    IssuerEmailAddress,
}

impl SslField {
    /// Returns a `&str` representation of an `SslField` enum
    pub fn as_str(&self) -> &str {
        match *self {
            SslField::IssuerSurname => "issuerSurname",
            SslField::SubjectOrganizationName => "subjectOrganizationName",
            SslField::IssuerCountry => "issuerCountry",
            SslField::IssuerOrganizationUnitName => "issuerOrganizationUnitName",
            SslField::Fingerprint => "fingerprint",
            SslField::SubjectOrganizationUnitName => "subjectOrganizationUnitName",
            SslField::SerialNumber => "serialNumber",
            SslField::SubjectEmailAddress => "subjectEmailAddress",
            SslField::SubjectCountry => "subjectCountry",
            SslField::IssuerGivenName => "issuerGivenName",
            SslField::SubjectCommonName => "subjectCommonName",
            SslField::IssuerCommonName => "issuerCommonName",
            SslField::IssuerStateOrProvinceName => "issuerStateOrProvinceName",
            SslField::IssuerProvince => "issuerProvince",
            SslField::SubjectStateOrProvinceName => "subjectStateOrProvinceName",
            SslField::Sha1 => "sha1",
            SslField::SubjectStreetAddress => "subjectStreetAddress",
            SslField::SubjectSerialNumber => "subjectSerialNumber",
            SslField::IssuerOrganizationName => "issuerOrganizationName",
            SslField::SubjectSurname => "subjectSurname",
            SslField::SubjectLocalityName => "subjectLocalityName",
            SslField::IssuerStreetAddress => "issuerStreetAddress",
            SslField::IssuerLocalityName => "issuerLocalityName",
            SslField::SubjectGivenName => "subjectGivenName",
            SslField::SubjectProvince => "subjectProvince",
            SslField::IssuerSerialNumber => "issuerSerialNumber",
            SslField::IssuerEmailAddress => "issuerEmailAddress",
        }
    }
}

impl FromStr for SslField {
    type Err = PassiveTotalError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "issuersurname" => Ok(SslField::IssuerSurname),
            "subjectorganizationname" => Ok(SslField::SubjectOrganizationName),
            "issuercountry" => Ok(SslField::IssuerCountry),
            "issuerorganizationunitname" => Ok(SslField::IssuerOrganizationUnitName),
            "fingerprint" => Ok(SslField::Fingerprint),
            "subjectorganizationunitname" => Ok(SslField::SubjectOrganizationUnitName),
            "serialnumber" => Ok(SslField::SerialNumber),
            "subjectemailaddress" => Ok(SslField::SubjectEmailAddress),
            "subjectcountry" => Ok(SslField::SubjectCountry),
            "issuergivenname" => Ok(SslField::IssuerGivenName),
            "subjectcommonname" => Ok(SslField::SubjectCommonName),
            "issuercommonname" => Ok(SslField::IssuerCommonName),
            "issuerstateorprovincename" => Ok(SslField::IssuerStateOrProvinceName),
            "issuerprovince" => Ok(SslField::IssuerProvince),
            "subjectstateorprovincename" => Ok(SslField::SubjectStateOrProvinceName),
            "sha1" => Ok(SslField::Sha1),
            "subjectstreetaddress" => Ok(SslField::SubjectStreetAddress),
            "subjectserialnumber" => Ok(SslField::SubjectSerialNumber),
            "issuerorganizationname" => Ok(SslField::IssuerOrganizationName),
            "subjectsurname" => Ok(SslField::SubjectSurname),
            "subjectlocalityname" => Ok(SslField::SubjectLocalityName),
            "issuerstreetaddress" => Ok(SslField::IssuerStreetAddress),
            "issuerlocalityname" => Ok(SslField::IssuerLocalityName),
            "subjectgivenname" => Ok(SslField::SubjectGivenName),
            "subjectprovince" => Ok(SslField::SubjectProvince),
            "issuerserialnumber" => Ok(SslField::IssuerSerialNumber),
            "issueremailaddress" => Ok(SslField::IssuerEmailAddress),
            _ => Err(PassiveTotalError::SslFieldParseError(String::from(s))),
        }
    }
}

pub struct SslRequest<'a> {
    pt: &'a PassiveTotal,
}

impl<'a> SslRequest<'a> {
    pub fn certificate(self, query: &'a str) -> SslCertificate {
        SslCertificate {
            pt: self.pt,
            url: URL_SSL,
            query: query,
        }
    }

    pub fn history(self, query: &'a str) -> SslHistory {
        SslHistory {
            pt: self.pt,
            url: URL_HISTORY,
            query: query,
        }
    }

    pub fn search(self, query: &'a str) -> SslSearch {
        SslSearch {
            pt: self.pt,
            query: query,
            field: None,
        }
    }
}

pub struct SslCertificate<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: &'a str,
}

pub struct SslHistory<'a> {
    pt: &'a PassiveTotal,
    url: &'a str,
    query: &'a str,
}

pub struct SslSearch<'a> {
    pt: &'a PassiveTotal,
    query: &'a str,
    field: Option<SslField>,
}

impl<'a> SslSearch<'a> {
    pub fn field(&'a mut self, field: SslField) -> &mut SslSearch {
        self.field = Some(field);
        self
    }

    pub fn send(&self) -> Result<Value> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        let url: &str;

        params.insert("query", self.query);
        if let Some(ref field) = self.field {
            url = URL_SEARCH;
            params.insert("field", field.as_str());
        } else {
            url = URL_KEYWORD;
        }

        self.pt.send_request_json_response(url, params)
    }
}

impl_send_query!(SslCertificate);
impl_send_query!(SslHistory);

impl PassiveTotal {
    pub fn ssl(&self) -> SslRequest {
        SslRequest {
            pt: self,
        }
    }
}

use std::str::FromStr;

use serde::{Serialize, Serializer};
use serde_json::Value;

use {PassiveTotal, PassiveTotalError, Result};

const URL_SSL: &str = "/ssl-certificate";
const URL_HISTORY: &str = "/ssl-certificate/history";
const URL_KEYWORD: &str = "/ssl-certificate/search/keyword";
const URL_SEARCH: &str = "/ssl-certificate/search";

pub struct SslRequest<'a> {
    pt: &'a PassiveTotal,
}

request_struct!(SslCertificate {
    query: &'a str,
});

request_struct!(SslHistory {
    query: &'a str
});

request_struct!(SslSearchField {
    query: &'a str,
    field: SslField,
});

request_struct!(SslSearchKeyword {
    query: &'a str
});

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

impl Serialize for SslField {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'a> SslRequest<'a> {
    pub fn certificate(self, query: &'a str) -> SslCertificate {
        SslCertificate {
            pt: self.pt,
            url: URL_SSL,
            query,
        }
    }

    pub fn history(self, query: &'a str) -> SslHistory {
        SslHistory {
            pt: self.pt,
            url: URL_HISTORY,
            query,
        }
    }

    pub fn search_keyword(self, query: &'a str) -> SslSearchKeyword {
        SslSearchKeyword {
            pt: self.pt,
            url: URL_KEYWORD,
            query,
        }
    }

    pub fn search_field(self, query: &'a str, field: SslField) -> SslSearchField {
        SslSearchField {
            pt: self.pt,
            url: URL_SEARCH,
            query,
            field,
        }
    }
}

impl_send!(SslCertificate);
impl_send!(SslHistory);
impl_send!(SslSearchField);
impl_send!(SslSearchKeyword);

impl PassiveTotal {
    pub fn ssl(&self) -> SslRequest {
        SslRequest { pt: self }
    }
}

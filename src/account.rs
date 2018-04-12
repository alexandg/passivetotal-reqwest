use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use serde_json::Value;

use {PassiveTotal, Result};

const URL_INFO: &str = "/account";
const URL_HISTORY: &str = "/account/history";
const URL_MONITORS: &str = "/account/monitors";
const URL_ORGANIZATION: &str = "/account/organization";
const URL_TEAMSTREAM: &str = "/account/organization/teamstream";
const URL_QUOTA: &str = "/account/quota";
const URL_SOURCES: &str = "/account/sources";

#[derive(Debug)]
struct PtDate {
    inner: DateTime<Utc>,
}

pub struct AccountRequest<'a> {
    pt: &'a PassiveTotal,
}

request_struct!(AccountQuota {});
request_struct!(AccountInfo {});
request_struct!(AccountHistory {});
request_struct!(AccountMonitors {});
request_struct!(AccountOrganization {});

request_struct!(AccountSources {
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
});

request_struct!(AccountTeamstream {
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datetime: Option<PtDate>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    typ: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    focus: Option<String>,
});

impl Serialize for PtDate {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dt_str = format!("{}", self.inner.format("%F %T"));
        serializer.serialize_str(&dt_str)
    }
}

impl<'a> AccountRequest<'a> {
    pub fn info(self) -> AccountInfo<'a> {
        AccountInfo {
            pt: self.pt,
            url: URL_INFO,
        }
    }

    pub fn history(self) -> AccountHistory<'a> {
        AccountHistory {
            pt: self.pt,
            url: URL_HISTORY,
        }
    }

    pub fn monitors(self) -> AccountMonitors<'a> {
        AccountMonitors {
            pt: self.pt,
            url: URL_MONITORS,
        }
    }

    pub fn organization(self) -> AccountOrganization<'a> {
        AccountOrganization {
            pt: self.pt,
            url: URL_ORGANIZATION,
        }
    }

    pub fn quota(self) -> AccountQuota<'a> {
        AccountQuota {
            pt: self.pt,
            url: URL_QUOTA,
        }
    }

    pub fn sources(self) -> AccountSources<'a> {
        AccountSources {
            pt: self.pt,
            url: URL_SOURCES,
            source: None,
        }
    }
}

impl<'a> AccountOrganization<'a> {
    pub fn teamstream(self) -> AccountTeamstream<'a> {
        AccountTeamstream {
            pt: self.pt,
            url: URL_TEAMSTREAM,
            source: None,
            datetime: None,
            typ: None,
            focus: None,
        }
    }
}

impl<'a> AccountTeamstream<'a> {
    pub fn source<S>(&mut self, source: S) -> &'a mut AccountTeamstream
    where
        S: Into<String>,
    {
        self.source = Some(source.into());
        self
    }

    pub fn datetime<S>(&mut self, dt: DateTime<Utc>) -> &'a mut AccountTeamstream {
        self.datetime = Some(PtDate { inner: dt });
        self
    }

    pub fn typ<S>(&mut self, ty: S) -> &'a mut AccountTeamstream
    where
        S: Into<String>,
    {
        self.typ = Some(ty.into());
        self
    }

    pub fn focus<S>(&mut self, focus: S) -> &'a mut AccountTeamstream
    where
        S: Into<String>,
    {
        self.focus = Some(focus.into());
        self
    }
}

impl<'a> AccountSources<'a> {
    pub fn source<S>(&mut self, source: S) -> &'a mut AccountSources
    where
        S: Into<String>,
    {
        self.source = Some(source.into());
        self
    }
}

impl_send!(AccountInfo);
impl_send!(AccountHistory);
impl_send!(AccountMonitors);
impl_send!(AccountOrganization);
impl_send!(AccountQuota);
impl_send!(AccountTeamstream);
impl_send!(AccountSources);

impl PassiveTotal {
    pub fn account(&self) -> AccountRequest {
        AccountRequest { pt: self }
    }
}

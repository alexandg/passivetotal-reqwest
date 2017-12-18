use {PassiveTotal, Result};

use chrono::{DateTime, Utc};
use serde_json::Value;

use std::collections::HashMap;

const URL_INFO: &str = "/account";
const URL_HISTORY: &str = "/account/history";
const URL_MONITORS: &str = "/account/monitors";
const URL_ORGANIZATION: &str = "/account/organization";
const URL_TEAMSTREAM: &str = "/account/organization/teamstream";
const URL_QUOTA: &str = "/account/quota";
const URL_SOURCES: &str = "/account/sources";

// XXX: This would probably be better as a custom derive so it can handle
// optional parameters automagically
macro_rules! impl_send {
    ($id:ident) => {
        impl<'a> $id<'a> {
            pub fn send(&self) -> Result<Value> {
                self.pt.send_request_json_response(self.url, json!({}))
            }
        }
    };
}

pub struct AccountRequest<'a> {
    pt: &'a PassiveTotal,
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

pub struct AccountInfo<'a> {
    pt: &'a PassiveTotal,
    url: &'static str,
}

pub struct AccountHistory<'a> {
    pt: &'a PassiveTotal,
    url: &'static str,
}

pub struct AccountMonitors<'a> {
    pt: &'a PassiveTotal,
    url: &'static str,
}

pub struct AccountOrganization<'a> {
    pt: &'a PassiveTotal,
    url: &'static str,
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

pub struct AccountTeamstream<'a> {
    pt: &'a PassiveTotal,
    url: &'static str,
    source: Option<String>,
    datetime: Option<DateTime<Utc>>,
    typ: Option<String>,
    focus: Option<String>,
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
        self.datetime = Some(dt);
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

    pub fn send(&self) -> Result<Value> {
        // Fix some borrowing issues
        let dt_str: String;
        let mut params: HashMap<&str, &str> = HashMap::new();

        if let Some(ref src) = self.source {
            params.insert("source", src);
        }

        if let Some(ref ty) = self.typ {
            params.insert("type", ty);
        }

        if let Some(ref focus) = self.focus {
            params.insert("focus", focus);
        }

        if let Some(ref dt) = self.datetime {
            dt_str = format!("{}", dt.format("%F %T"));
            params.insert("dt", &dt_str);
        }

        self.pt.send_request_json_response(self.url, params)
    }
}

pub struct AccountQuota<'a> {
    pt: &'a PassiveTotal,
    url: &'static str,
}

pub struct AccountSources<'a> {
    pt: &'a PassiveTotal,
    url: &'static str,
    source: Option<String>,
}

impl<'a> AccountSources<'a> {
    pub fn source<S>(&mut self, source: S) -> &'a mut AccountSources
    where
        S: Into<String>,
    {
        self.source = Some(source.into());
        self
    }

    pub fn send(&self) -> Result<Value> {
        let params = if let Some(ref src) = self.source {
            json!({
                "source": src
            })
        } else {
            json!({})
        };

        self.pt.send_request_json_response(self.url, params)
    }
}

impl PassiveTotal {
    pub fn account(&self) -> AccountRequest {
        AccountRequest { pt: self }
    }
}

impl_send!(AccountInfo);
impl_send!(AccountHistory);
impl_send!(AccountMonitors);
impl_send!(AccountOrganization);
impl_send!(AccountQuota);

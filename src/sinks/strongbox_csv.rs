use anyhow::Result;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::__private::from_utf8_lossy;
use std::{borrow::Cow, io::Write};

use super::Sink;
use crate::{sources::Source, universal::UniversalItem};

#[derive(serde::Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct StrongboxCsvItem<'a> {
    pub title: Cow<'a, str>,
    pub username: Option<Cow<'a, str>>,
    pub email: Option<Cow<'a, str>>,
    #[serde(rename = "URL")]
    pub url: Option<Cow<'a, str>>,
    pub password: Option<Cow<'a, str>>,
    pub phone: Option<Cow<'a, str>>,
    #[serde(rename = "OTPAuth")]
    pub otpauth: Option<Cow<'a, str>>,
    pub notes: Cow<'a, str>,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
}

impl<'a, 'b> From<UniversalItem<'a>> for StrongboxCsvItem<'b>
where
    'a: 'b,
{
    fn from(item: UniversalItem<'a>) -> Self {
        let mut converted = StrongboxCsvItem {
            title: item.title,
            username: item.username,
            email: item.email,
            url: item.url,
            password: item.password,
            phone: item.phone,
            otpauth: item.otpauth,
            notes: item.notes,
            created: item.created,
        };

        for (name, value) in item.unknown_fields {
            let str = from_utf8_lossy(value.as_ref());
            if !str.is_empty() {
                converted.notes = Cow::Owned(format!("{}\n{}: {}", converted.notes, name, str));
            }
        }

        if let Some(ref email) = converted.email {
            if converted.username.is_none() {
                converted.username = Some(email.clone());
            }
        }

        if let Some(phone) = converted.phone.as_ref() {
            if converted.username.is_some() {
                converted.notes = Cow::Owned(format!("{}\nPhone: {phone}", converted.notes));
            } else {
                converted.username = converted.phone.clone();
            }
        }

        converted
    }
}

pub struct Strongbox<W: Write>(csv::Writer<W>);
impl<W: Write> Strongbox<W> {
    pub fn new(out: W) -> Self {
        Self(csv::Writer::from_writer(out))
    }
}

impl<W: Write> Sink for Strongbox<W> {
    fn user_friendly_name() -> &'static str {
        "strongbox"
    }

    fn write<'i>(&mut self, items: impl Source<'i>) -> Result<()> {
        for item in items {
            self.0.serialize(StrongboxCsvItem::from(item))?;
        }
        self.0.flush()?;
        Ok(())
    }
}

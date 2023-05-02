use anyhow::Context;
use chrono::{serde::ts_seconds, DateTime, Utc};
use std::borrow::Cow;
use std::fmt::Write;

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

impl<'a, 'b> TryFrom<UniversalItem<'a>> for StrongboxCsvItem<'b>
where
    'a: 'b,
{
    type Error = anyhow::Error;
    fn try_from(item: UniversalItem<'a>) -> Result<Self, Self::Error> {
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
            let str = std::str::from_utf8(value.as_ref());
            if let Ok(str) = str {
                if !str.is_empty() {
                    converted.notes = Cow::Owned(format!("{}\n{}: {}", converted.notes, name, str));
                }
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

        Ok(converted)
    }
}

pub struct Strongbox<W: std::io::Write>(csv::Writer<W>);
impl<W: std::io::Write> Strongbox<W> {
    pub fn with_output(out: W) -> Self {
        Self(csv::Writer::from_writer(out))
    }
}

impl<W: std::io::Write> Sink for Strongbox<W> {
    fn convert_from_source(&mut self, source: Box<dyn Source>) -> anyhow::Result<()> {
        let mut error_context = String::new();
        for item in source.into_item_iter() {
            error_context.clear();
            write!(error_context, "Converting item {item} to Strongbox")?;
            let converted_item =
                StrongboxCsvItem::try_from(item).with_context(|| error_context.clone())?;
            self.0
                .serialize(converted_item)
                .context("Serializing to output")?;
        }
        self.0.flush()?;
        Ok(())
    }
}

use chrono::{serde::ts_seconds, DateTime, Utc};
use std::fmt::Write;
use std::{borrow::Cow, io};

use super::Sink;
use crate::{sources::Source, universal::UniversalItem};

#[derive(serde::Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct StrongboxCsvItem<'item> {
    pub title: Cow<'item, str>,
    pub username: Option<Cow<'item, str>>,
    pub email: Option<Cow<'item, str>>,
    #[serde(rename = "URL")]
    pub url: Option<Cow<'item, str>>,
    pub password: Option<Cow<'item, str>>,
    pub phone: Option<Cow<'item, str>>,
    #[serde(rename = "OTPAuth")]
    pub otpauth: Option<Cow<'item, str>>,
    pub notes: Cow<'item, str>,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum ItemErrorKind {
    CantConvert(#[from] anyhow::Error),
    CantSerializeToCsv(#[from] csv::Error),
    Io(#[from] io::Error),
    Fmt(#[from] std::fmt::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("Unable to convert item {item_title} to Strongbox")]
struct ItemError {
    item_title: String,
    source: ItemErrorKind,
}

impl<'a, 'b> TryFrom<UniversalItem<'a>> for StrongboxCsvItem<'b>
where
    'a: 'b,
{
    type Error = ItemErrorKind;
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
            if let Ok(str) = std::str::from_utf8(value.as_ref()) {
                if !str.is_empty() {
                    write!(converted.notes.to_mut(), "\n{name}: {str}")?;
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
                write!(converted.notes.to_mut(), "\nPhone: {phone}")?;
            } else {
                converted.username = converted.phone.clone();
            }
        }

        Ok(converted)
    }
}

pub struct Strongbox<W: io::Write>(csv::Writer<W>);
impl<W: io::Write> Strongbox<W> {
    pub fn with_output(out: W) -> Self {
        Self(csv::Writer::from_writer(out))
    }

    fn serialize_item(&mut self, item: UniversalItem) -> Result<(), ItemErrorKind> {
        self.0.serialize(StrongboxCsvItem::try_from(item)?)?;
        Ok(())
    }
}

impl<W: io::Write> Sink for Strongbox<W> {
    fn digest_items(&mut self, source: Box<dyn Source>) -> Result<(), anyhow::Error> {
        for item in source.into_item_iter() {
            let item_title = item.title.clone();
            self.serialize_item(item).map_err(|source| ItemError {
                item_title: item_title.into_owned(),
                source: source.into(),
            })?;
        }
        self.0.flush()?;
        Ok(())
    }
}

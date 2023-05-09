use crate::conversion::UniversalItem;
use anyhow::Context;
use chrono::{serde::ts_seconds, DateTime, Utc};
use std::{borrow::Cow, io::BufRead};

#[derive(serde::Deserialize)]
pub struct EnpassJson<'a> {
    pub items: Vec<EnpassItem<'a>>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnpassItem<'a> {
    pub title: Cow<'a, str>,
    pub note: Cow<'a, str>,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub fields: Option<Vec<EnpassField<'a>>>,
}

#[derive(serde::Deserialize)]
pub struct EnpassField<'a> {
    pub label: Cow<'a, str>,
    pub r#type: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

pub struct EnpassIterator<'a>(std::vec::IntoIter<EnpassItem<'a>>);
impl<'a> Iterator for EnpassIterator<'a> {
    type Item = UniversalItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|item| {
            let mut converted = UniversalItem {
                title: item.title,
                notes: item.note,
                created: item.created_at,
                ..Default::default()
            };

            for field in item
                .fields
                .into_iter()
                .flatten()
                .filter(|field| !field.value.is_empty())
            {
                match field.r#type.as_ref() {
                    "username" => converted.username = Some(field.value),
                    "email" => converted.email = Some(field.value),
                    "url" => converted.url = Some(field.value),
                    "password" => converted.password = Some(field.value),
                    "totp" => converted.otpauth = Some(field.value),
                    "phone" => converted.phone = Some(field.value),
                    "text" if field.label.contains("phone") && converted.phone.is_none() => {
                        converted.phone = Some(field.value)
                    }
                    _ => converted
                        .unknown_fields
                        .push((field.label, Cow::Owned(field.value.as_bytes().to_vec()))),
                }
            }

            converted
        })
    }
}

impl<'a> super::Source<'a> for EnpassJson<'a> {
    fn from_reader(reader: impl BufRead) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        serde_json::from_reader(reader).context("Unable to parse the enpass .json export")
    }
    fn into_item_iter(self: Box<Self>) -> Box<dyn Iterator<Item = UniversalItem<'a>> + 'a> {
        Box::new(EnpassIterator(self.items.into_iter()))
    }
}

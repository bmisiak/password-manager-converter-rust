use chrono::{serde::ts_seconds, DateTime, Utc};
use std::borrow::Cow;
#[derive(serde::Deserialize)]
pub struct EnpassExport<'a> {
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

impl<'a> IntoIterator for EnpassExport<'a> {
    type Item = EnpassItem<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a> crate::source::Source for EnpassExport<'a> {}

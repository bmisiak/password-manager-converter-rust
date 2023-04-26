use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Default)]
pub struct UniversalItem<'a> {
    pub title: Cow<'a, str>,
    pub username: Option<Cow<'a, str>>,
    pub email: Option<Cow<'a, str>>,
    pub url: Option<Cow<'a, str>>,
    pub password: Option<Cow<'a, str>>,
    pub phone: Option<Cow<'a, str>>,
    pub otpauth: Option<Cow<'a, str>>,
    pub notes: Cow<'a, str>,
    pub created: DateTime<Utc>,
    pub unknown_fields: Vec<(Cow<'a, str>, Cow<'a, [u8]>)>,
}

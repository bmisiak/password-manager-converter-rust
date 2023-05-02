use chrono::{DateTime, Utc};
use std::{borrow::Cow, fmt::Display};

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

impl<'a> Display for UniversalItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\ntitle: {},\nusername: {}\n}}",
            self.title,
            self.username.as_ref().unwrap_or(&Cow::Borrowed("-"))
        )?;
        Ok(())
    }
}

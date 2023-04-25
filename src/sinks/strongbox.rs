use chrono::{serde::ts_seconds, DateTime, Utc};
use std::borrow::Cow;

use crate::sources::enpass_json::EnpassItem;

#[derive(serde::Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct StrongboxFormat<'a> {
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

impl<'a, 'b> From<EnpassItem<'a>> for StrongboxFormat<'b>
where
    'a: 'b,
{
    fn from(item: EnpassItem<'a>) -> Self {
        let mut converted = StrongboxFormat {
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
                _ => {
                    converted.notes = Cow::Owned(format!(
                        "{}\n{}: {}",
                        converted.notes,
                        field.label,
                        field.value.trim()
                    ));
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

        converted
    }
}

use chrono::{DateTime, Utc};
use std::{
    borrow::Cow,
    fmt::{self, Display},
    fs::File,
    io::{stdin, BufRead, BufReader},
};

use crate::sinks::{strongbox_csv::Strongbox, Sink};
use crate::sources::{enpass_json::EnpassJson, Source};
use anyhow::{bail, Context};

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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Item")
            .field("title", &self.title.as_ref())
            .field("username", &self.username.as_deref().unwrap_or("-"))
            .finish()
    }
}

pub fn run_conversion(
    source_name: &str,
    source_path: &str,
    sink_name: &str,
    sink_path: &str,
) -> Result<(), anyhow::Error> {
    let source_reader: Box<dyn BufRead> = if source_name == "stdin" {
        Box::new(BufReader::new(stdin()))
    } else {
        Box::new(BufReader::new(
            File::open(source_path).context(format!("Unable to read {source_path}"))?,
        ))
    };
    let sink_destination = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(sink_path)
        .context(format!("Unable to open {sink_path}"))?;
    let source = match source_name {
        "enpass" => Box::new(EnpassJson::from_reader(source_reader)?),
        _ => bail!("Unsupported source name"),
    };
    let mut sink = match sink_name {
        "strongbox" => Box::new(Strongbox::with_output(sink_destination)),
        _ => bail!("Unsupported target name"),
    };
    sink.digest_items(source)?;
    Ok(())
}

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::env::args;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

use crate::sinks::strongbox_csv::Strongbox;
use crate::sources::enpass_json::EnpassJson;
use sinks::Sink;
use sources::Source;

mod sinks;
mod sources;
mod universal;

fn main() -> Result<()> {
    let mut args = args();
    let invocation = args.next().unwrap();
    if let Some((source_name, source_path, _to, sink_name, sink_path)) = args.next_tuple() {
        let source_reader: Box<dyn BufRead> = if source_name == "stdin" {
            Box::new(BufReader::new(stdin()))
        } else {
            Box::new(BufReader::new(
                File::open(&source_path).context(format!("Unable to read {source_path}"))?,
            ))
        };

        let sink_destination = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&sink_path)
            .context(format!("Unable to open {sink_path}"))?;

        let source: Box<dyn Source> = match source_name.as_str() {
            "enpass" => Box::new(EnpassJson::from_reader(source_reader)?),
            _ => bail!("Unsupported source name"),
        };

        let mut sink: Box<dyn Sink> = match sink_name.as_str() {
            "strongbox" => Box::new(Strongbox::with_output(sink_destination)),
            _ => bail!("Unsupported target name"),
        };

        sink.digest_items(source)?;

        println!("Conversion successful!");
    } else {
        println!("Usage: {invocation} enpass SOURCE_FILE to strongbox OUTPUT_FILE");
        println!("Sample usage: {invocation} enpass ./export.json to strongbox ./out.csv");
    }
    Ok(())
}

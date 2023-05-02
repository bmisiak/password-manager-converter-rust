use anyhow::{bail, Context, Result};
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let (source_name, source_reader, sink_name, sink_destination) = match (
        args.next(),
        args.next(),
        args.next().as_deref(),
        args.next(),
        args.next(),
    ) {
        (Some(source_name), Some(source_path), Some("to"), Some(sink_name), Some(sink_path)) => {
            let source_reader: Box<dyn BufRead> = if source_name == "stdin" {
                Box::new(BufReader::new(std::io::stdin()))
            } else {
                Box::new(BufReader::new(
                    File::open(&source_path)
                        .with_context(|| format!("Unable to read {source_path}"))?,
                ))
            };

            let sink_destination = File::options()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&sink_path)
                .with_context(|| format!("Unable to open {sink_path}"))?;

            (source_name, source_reader, sink_name, sink_destination)
        }
        _ => {
            println!("Usage: {invocation} enpass SOURCE_FILE to strongbox OUTPUT_FILE");
            println!("Sample usage: {invocation} enpass ./export.json to strongbox ./out.csv");
            return Ok(());
        }
    };

    let source: Box<dyn Source> = match source_name.as_str() {
        "enpass" => Box::new(
            EnpassJson::from_reader(source_reader)
                .with_context(|| format!("Unable to parse the enpass .json export"))?,
        ),
        _ => bail!("Unsupported source name"),
    };

    let mut sink: Box<dyn Sink> = match sink_name.as_str() {
        "strongbox" => Box::new(Strongbox::with_output(sink_destination)),
        _ => bail!("Unsupported target name"),
    };

    sink.convert_from_source(source)
        .with_context(|| format!("Unable to convert to {sink_name}"))?;

    println!("Conversion successful!");

    Ok(())
}

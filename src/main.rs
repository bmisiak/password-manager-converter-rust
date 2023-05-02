use anyhow::{bail, Context, Result};
use sources::Source;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::sinks::strongbox_csv::Strongbox;
use crate::sinks::Sink;
use crate::sources::enpass_json::EnpassJson;

mod sinks;
mod sources;
mod universal;

fn main() -> Result<()> {
    let reader: Box<dyn BufRead> = match std::env::args().nth(1).as_deref() {
        Some("-") => Box::new(BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(BufReader::new(
            File::open(filename).with_context(|| format!("Unable to read {filename}"))?,
        )),
        None => {
            bail!("Export .json from your password manager and provide the path to the file as an argument, or \"-\" to read from stdin.");
        }
    };

    let source_name = "enpass";
    let source: Box<dyn Source> = match source_name {
        "enpass" => Box::new(EnpassJson::from_reader(reader)?),
        _ => bail!("Unsupported source name"),
    };

    let sink_path = "./output.csv";
    let sink_destination =
        File::open(sink_path).with_context(|| format!("Unable to open {sink_path}"))?;
    let sink_name = "strongbox";
    let mut sink: Box<dyn Sink> = match sink_name {
        "strongbox" => Box::new(Strongbox::with_output(sink_destination)),
        _ => bail!("Unsupported target name"),
    };

    sink.convert_from_source(source)
        .with_context(|| format!("Conversion from {source_name} to {sink_name}"))?;

    println!("Saved to output.csv");

    Ok(())
}

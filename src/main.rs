use anyhow::{anyhow, Context, Result};
use std::fs::File;
use std::io::BufReader;

use crate::sinks::strongbox_csv::Strongbox;
use crate::sinks::Sink;
use crate::sources::enpass_json::EnpassJson;

mod sinks;
mod sources;
mod universal;

fn main() -> Result<()> {
    let reader: Box<dyn std::io::BufRead> = match std::env::args().nth(1).as_deref() {
        Some("-") => Box::new(BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(BufReader::new(
            File::open(filename).with_context(|| format!("Unable to read {filename}"))?,
        )),
        None => {
            return Err(anyhow!("Export .json from your password manager and provide the path to the file as an argument, or \"-\" to read from stdin."));
        }
    };

    let source: EnpassJson =
        serde_json::from_reader(reader).context("The input file has an incorrect format")?;

    let output_file = File::open("./output.csv").context("Unable to write to ./output.csv")?;
    let mut sink = Strongbox::new(output_file);
    sink.write(source).context("Failed writing to output")?;
    println!("Saved to output.csv");

    Ok(())
}

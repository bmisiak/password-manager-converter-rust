#![feature(let_chains)]
use anyhow::Result;
use itertools::Itertools;
use std::env::args;

use crate::conversion::run_conversion;

mod conversion;
mod sinks;
mod sources;

fn main() -> Result<()> {
    let mut args = args();
    let invocation = args.next().unwrap();
    if let Some((source_name, source_path, _to, sink_name, sink_path)) = args.next_tuple() {
        run_conversion(&source_name, &source_path, &sink_name, &sink_path)?;
        println!("Conversion successful!");
    } else {
        println!("Usage: {invocation} enpass SOURCE_FILE to strongbox OUTPUT_FILE");
        println!("Example: {invocation} enpass ./export.json to strongbox ./out.csv");
    }
    Ok(())
}

use std::fs::File;
use std::io::BufReader;

mod sinks;
mod source;
mod sources;

fn main() {
    let mut writer = csv::Writer::from_path("./output.csv").unwrap();
    let reader: Box<dyn std::io::BufRead> = match std::env::args().nth(1).as_deref() {
        Some("-") => Box::new(BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
        None => {
            println!("Export .json from Enpass and provide the path to the file as an argument, or \"-\" to read from stdin.");
            return;
        }
    };

    let export: sources::enpass_json::EnpassExport = serde_json::from_reader(reader).unwrap();

    for item in export
        .into_iter()
        .map(sinks::strongbox::StrongboxFormat::from)
    {
        writer.serialize(item).unwrap();
    }
    writer.flush().unwrap();
    println!("Saved to output.csv");
}

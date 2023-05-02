use crate::sources::Source;

pub mod strongbox_csv;

pub trait Sink {
    fn convert_from_source(&mut self, items: Box<dyn Source>) -> anyhow::Result<()>;
}

use crate::sources::Source;

pub mod strongbox_csv;

pub trait Sink {
    fn digest_items(&mut self, items: Box<dyn Source>) -> anyhow::Result<()>;
}

use crate::sources::Source;

pub mod strongbox_csv;

pub trait Sink {
    fn user_friendly_name() -> &'static str;
    fn write<'i>(&mut self, items: impl Source<'i>) -> anyhow::Result<()>;
}

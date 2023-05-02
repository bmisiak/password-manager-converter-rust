use std::io::BufRead;

use crate::universal::UniversalItem;

pub mod enpass_json;

pub trait Source<'a> {
    fn from_reader<R: BufRead>(reader: R) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn into_item_iter(self: Box<Self>) -> Box<dyn Iterator<Item = UniversalItem<'a>> + 'a>;
}

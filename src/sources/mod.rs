use crate::universal::UniversalItem;

pub mod enpass_json;

pub trait Source<'a>
where
    Self: IntoIterator<Item = UniversalItem<'a>, IntoIter = Self::Itr>,
{
    type Itr: Iterator<Item = UniversalItem<'a>>;
}

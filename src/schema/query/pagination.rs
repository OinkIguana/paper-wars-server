use super::{Context, QueryWrapper};
use data::Searchable;

pub struct Pagination<T>
where
    T: QueryWrapper,
    T::Model: Searchable,
{
    pub search: <T::Model as Searchable>::Search,
    pub items: Vec<T>,
}

impl<T> Pagination<T>
where
    T: QueryWrapper,
    T::Model: Searchable,
{
    pub fn new(
        search: <T::Model as Searchable>::Search,
        items: impl IntoIterator<Item = T>,
    ) -> Self {
        Self {
            search,
            items: items.into_iter().collect(),
        }
    }

    pub fn items(&self) -> &[T] {
        self.items.as_slice()
    }

    pub fn total(&self) -> i32 {
        0
    }

    pub fn start(&self, context: &Context) -> juniper::FieldResult<Option<String>> {
        let item = match self.items.first() {
            Some(item) => item,
            None => return Ok(None),
        };
        Ok(Some(item.load(context)?.cursor(&self.search, 0usize)))
    }

    pub fn end(&self, context: &Context) -> juniper::FieldResult<Option<String>> {
        let item = match self.items.last() {
            Some(item) => item,
            None => return Ok(None),
        };
        Ok(Some(
            item.load(context)?
                .cursor(&self.search, self.items.len()),
        ))
    }
}

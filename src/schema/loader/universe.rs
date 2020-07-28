use super::Loader;
use data::{universes, Universe};
use diesel::prelude::*;
use diesel_citext::prelude::*;
use tokio::task;
use uuid::Uuid;

batch_fn!(universes => Universe { id: Uuid });

impl Loader<Uuid, Universe> {
    pub async fn search(&self, search: Option<UniverseSearch>) -> Vec<Universe> {
        let load_result: anyhow::Result<Vec<Universe>> = task::block_in_place(|| {
            let conn = self.database.connection()?;
            let mut query = universes::table.into_boxed();
            if let Some(search) = search {
                if let Some(search_name) = search.name {
                    query = query.filter(universes::name.like(CiString::from(format!("%{}%", search_name))));
                }
                if let Some(limit) = search.limit {
                    query = query.limit(limit as i64);
                }
                if let Some(offset) = search.cursor {
                    query = query.offset(offset as i64);
                }
            }
            Ok(query.load(&conn)?)
        });

        let items = load_result.unwrap_or(vec![]);
        let to_cache = items
            .iter()
            .cloned()
            .map(|item| (item.id.to_owned(), Some(item)));
        self.prime_many(to_cache).await;
        items
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct UniverseSearch {
    name: Option<String>,
    limit: Option<i32>,
    cursor: Option<i32>,
}

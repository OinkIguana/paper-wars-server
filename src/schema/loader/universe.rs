use super::Loader;
use diesel::prelude::*;
use diesel_citext::prelude::*;
use tokio::task;
use data::{Universe, universes::dsl::*};
use uuid::Uuid;

loader_by_id!(Uuid, Universe, universes);

impl Loader<Uuid, Universe> {
    pub async fn search(&self, search: Option<UniverseSearch>) -> Vec<Universe> {
        let load_result: anyhow::Result<Vec<Universe>> = task::block_in_place(|| {
            let conn = self.database.connection()?;
            let mut query = universes.into_boxed();
            if let Some(search) = search {
                if let Some(search_name) = search.name {
                    query = query
                        .filter(name.like(CiString::from(format!("%{}%", search_name))));
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
        for item in &items {
            self.loader
                .prime(item.id.clone(), Some(item.clone()))
                .await;
        }
        items
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct UniverseSearch {
    name: Option<String>,
    limit: Option<i32>,
    cursor: Option<i32>,
}

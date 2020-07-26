use async_trait::async_trait;
use dataloader::BatchFn;
use diesel::prelude::*;
use std::collections::HashMap;
use tokio::task;
use uuid::Uuid;

#[derive(Clone)]
pub struct Database(data::Database);

impl Database {
    pub fn connect(database_url: String) -> anyhow::Result<Self> {
        data::Database::connect(database_url).map(Self)
    }
}

macro_rules! loader_by_id {
    ($model:ty, $table:ident) => {
        #[async_trait]
        impl BatchFn<Uuid, Option<$model>> for Database {
            async fn load(&self, keys: &[Uuid]) -> HashMap<Uuid, Option<$model>> {
                let mut map: HashMap<Uuid, Option<$model>> = keys.iter()
                    .cloned()
                    .map(|key| (key, None))
                    .collect();
                let load_result: anyhow::Result<Vec<$model>> = task::block_in_place(|| {
                    let conn = self.0.connection()?;
                    Ok(data::$table::table
                        .filter(data::$table::id.eq_any(keys))
                        .load(&conn)?)
                });
                let items = load_result.unwrap_or(vec![]);
                for item in items {
                    map.get_mut(&item.id).unwrap().replace(item);
                }
                map
            }
        }
    };
}

loader_by_id!(data::Universe, universes);
loader_by_id!(data::Account, accounts);

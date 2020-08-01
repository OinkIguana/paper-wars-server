use super::Database;
use data::{Searchable, TryAsQuery};
use dataloader::BatchFn;
use diesel::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;
use uuid::Uuid;

mod traits;
#[macro_use]
mod macros;

mod account;
mod archetype;
mod archetype_version;
mod contributor;
mod email;
mod entity;
mod login;
mod map;
mod map_version;
mod player;
mod universe_version;
mod universe_version_archetype;
mod universe_version_map;

pub struct Loader<K, T>
where
    K: Hash + Eq + Clone + Debug,
    T: Clone + Debug,
    Database: BatchFn<K, Option<T>>,
{
    loader: dataloader::cached::Loader<K, Option<T>, Database>,
    database: Database,
}

impl<K, T> Loader<K, T>
where
    K: Hash + Eq + Clone + Debug,
    T: Clone + Debug,
    Database: BatchFn<K, Option<T>>,
{
    pub fn new(database: Database) -> Self {
        Self {
            loader: dataloader::cached::Loader::new(database.clone()),
            database,
        }
    }

    pub async fn load(&self, key: K) -> Option<T> {
        self.loader.load(key).await
    }

    #[allow(dead_code)]
    pub async fn load_many(&self, keys: Vec<K>) -> std::collections::HashMap<K, Option<T>> {
        self.loader.load_many(keys).await
    }

    pub async fn prime(&self, item: T)
    where
        T: traits::BatchFnItem<Key = K>,
    {
        self.loader
            .prime(traits::BatchFnItem::key(&item), Some(item))
            .await
    }

    async fn prime_many(&self, items: impl IntoIterator<Item = T>)
    where
        T: traits::BatchFnItem<Key = K>,
    {
        for item in items {
            self.loader
                .prime(traits::BatchFnItem::key(&item), Some(item))
                .await;
        }
    }
}

impl<K, T> Loader<K, T>
where
    K: Hash + Eq + Clone + Debug,
    T: Clone + Debug + Searchable + traits::BatchFnItem<Key = K>,
    Database: BatchFn<K, Option<T>>,
{
    pub async fn search(&self, search: &T::Search) -> anyhow::Result<Vec<T>> {
        let items = tokio::task::block_in_place(|| {
            let conn = self.database.connection()?;
            let query = search.try_as_query()?;
            anyhow::Result::<Vec<T>>::Ok(query.load(&conn)?)
        })?;
        self.prime_many(items.clone()).await;
        Ok(items)
    }
}

batch_fn!(universes => data::Universe { id: Uuid });
batch_fn!(games => data::Game { id: Uuid });

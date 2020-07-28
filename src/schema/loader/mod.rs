use super::Database;
use dataloader::BatchFn;
use std::fmt::Debug;
use std::hash::Hash;

mod traits;
#[macro_use]
mod macros;

mod account;
mod contributor;
mod email;
mod universe;

pub use account::*;
pub use universe::*;

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

    async fn prime(&self, key: K, value: Option<T>) {
        self.loader.prime(key, value).await;
    }

    async fn prime_many(&self, items: impl IntoIterator<Item = T>)
    where
        T: traits::BatchFnItem<Key = K>,
    {
        for item in items {
            self.prime(traits::BatchFnItem::key(&item), Some(item))
                .await;
        }
    }
}

batch_fn!(logins => data::Login { account_id: uuid::Uuid });

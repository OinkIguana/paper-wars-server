use super::Database;
use dataloader::BatchFn;
use std::fmt::Debug;
use std::hash::Hash;

#[macro_use]
mod macros;

mod account;
mod emails;
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

    pub async fn prime(&self, key: K, value: Option<T>) {
        self.loader.prime(key, value).await;
    }

    pub async fn prime_many(&self, items: impl IntoIterator<Item = (K, Option<T>)>) {
        for (key, value) in items {
            self.prime(key, value).await;
        }
    }
}

batch_fn!(uuid::Uuid, data::Login, logins, account_id);

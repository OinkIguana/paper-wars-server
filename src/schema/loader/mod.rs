use super::Database;
use dataloader::BatchFn;
use std::fmt::Debug;
use std::hash::Hash;
use uuid::Uuid;

#[macro_use]
mod macros;

mod universe;
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
}

loader_by_id!(Uuid, data::Account, accounts);

use super::Database;
use data::{Account, Universe};
use uuid::Uuid;

type Loader<T> = dataloader::cached::Loader<Uuid, Option<T>, Database>;

pub struct Context {
    universe_loader: Loader<Universe>,
    account_loader: Loader<Account>,
}

impl Context {
    pub fn new(database: Database) -> Self {
        Self {
            universe_loader: Loader::new(database.clone()),
            account_loader: Loader::new(database),
        }
    }

    pub fn universes(&self) -> &Loader<Universe> {
        &self.universe_loader
    }

    pub fn accounts(&self) -> &Loader<Account> {
        &self.account_loader
    }
}

impl juniper::Context for Context {}

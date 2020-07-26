use super::{Database, Loader};
use data::{Account, Universe};
use uuid::Uuid;

pub struct Context {
    universe_loader: Loader<Uuid, Universe>,
    account_loader: Loader<Uuid, Account>,
}

impl Context {
    pub fn new(database: Database) -> Self {
        Self {
            universe_loader: Loader::new(database.clone()),
            account_loader: Loader::new(database),
        }
    }

    pub fn universes(&self) -> &Loader<Uuid, Universe> {
        &self.universe_loader
    }

    pub fn accounts(&self) -> &Loader<Uuid, Account> {
        &self.account_loader
    }
}

impl juniper::Context for Context {}

use super::{Database, Loader};
use data::*;
use diesel_citext::types::CiString;
use uuid::Uuid;

pub struct Context {
    universe_loader: Loader<Uuid, Universe>,
    account_loader: Loader<Uuid, Account>,
    email_loader: Loader<CiString, Email>,
    login_loader: Loader<Uuid, Login>,
}

impl Context {
    pub fn new(database: Database) -> Self {
        Self {
            universe_loader: Loader::new(database.clone()),
            account_loader: Loader::new(database.clone()),
            email_loader: Loader::new(database.clone()),
            login_loader: Loader::new(database),
        }
    }

    pub fn universes(&self) -> &Loader<Uuid, Universe> {
        &self.universe_loader
    }

    pub fn accounts(&self) -> &Loader<Uuid, Account> {
        &self.account_loader
    }

    pub fn emails(&self) -> &Loader<CiString, Email> {
        &self.email_loader
    }

    pub fn logins(&self) -> &Loader<Uuid, Login> {
        &self.login_loader
    }
}

impl juniper::Context for Context {}

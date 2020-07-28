use super::{Database, Loader};
use data::*;
use diesel_citext::types::CiString;
use uuid::Uuid;

pub struct Context {
    account_loader: Loader<Uuid, Account>,
    archetype_loader: Loader<Uuid, Archetype>,
    contributor_loader: Loader<(Uuid, Uuid), Contributor>,
    email_loader: Loader<CiString, Email>,
    login_loader: Loader<Uuid, Login>,
    universe_loader: Loader<Uuid, Universe>,
}

impl Context {
    pub fn new(database: Database) -> Self {
        Self {
            account_loader: Loader::new(database.clone()),
            archetype_loader: Loader::new(database.clone()),
            contributor_loader: Loader::new(database.clone()),
            email_loader: Loader::new(database.clone()),
            login_loader: Loader::new(database.clone()),
            universe_loader: Loader::new(database),
        }
    }

    pub fn accounts(&self) -> &Loader<Uuid, Account> {
        &self.account_loader
    }

    pub fn archetypes(&self) -> &Loader<Uuid, Archetype> {
        &self.archetype_loader
    }

    pub fn contributors(&self) -> &Loader<(Uuid, Uuid), Contributor> {
        &self.contributor_loader
    }

    pub fn emails(&self) -> &Loader<CiString, Email> {
        &self.email_loader
    }

    pub fn logins(&self) -> &Loader<Uuid, Login> {
        &self.login_loader
    }

    pub fn universes(&self) -> &Loader<Uuid, Universe> {
        &self.universe_loader
    }
}

impl juniper::Context for Context {}

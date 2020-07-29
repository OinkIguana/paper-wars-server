use super::loader::{AccountSearch, UniverseSearch};
use super::Context;
use uuid::Uuid;

mod account;
mod archetype;
mod archetype_version;
mod contributor;
mod email;
mod game;
mod map;
mod map_version;
mod universe;
mod universe_version;

use account::Account;
use archetype::Archetype;
use archetype_version::ArchetypeVersion;
use contributor::Contributor;
use email::Email;
use game::Game;
use map::Map;
use map_version::MapVersion;
use universe::Universe;
use universe_version::UniverseVersion;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    /// Current version of this API.
    fn version() -> i32 {
        1
    }

    /// Game universes, created by users.
    async fn universes(
        context: &Context,
        universes: Option<Vec<Uuid>>,
        search: Option<UniverseSearch>,
    ) -> Vec<Option<Universe>> {
        if let Some(universes) = universes {
            return universes.into_iter().map(Universe::new).map(Some).collect();
        }
        context
            .universes()
            .search(search)
            .await
            .into_iter()
            .map(|universe| Universe::new(universe.id))
            .map(Some)
            .collect()
    }

    /// User accounts.
    async fn accounts(
        context: &Context,
        accounts: Option<Vec<Uuid>>,
        search: Option<AccountSearch>,
    ) -> Vec<Option<Account>> {
        if let Some(accounts) = accounts {
            return accounts.into_iter().map(Account::new).map(Some).collect();
        }
        context
            .accounts()
            .search(search)
            .await
            .into_iter()
            .map(|account| Account::new(account.id))
            .map(Some)
            .collect()
    }
}

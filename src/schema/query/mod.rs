use super::Context;
use juniper::FieldResult;
use uuid::Uuid;

mod traits;
pub use traits::QueryWrapper;

#[macro_use]
mod pagination;
use pagination::Pagination;

mod account;
mod archetype;
mod archetype_version;
mod contributor;
mod email;
mod entity;
mod game;
mod map;
mod map_version;
mod player;
mod universe;
mod universe_version;

use account::Account;
use archetype::Archetype;
use archetype_version::ArchetypeVersion;
use contributor::Contributor;
use email::Email;
use entity::Entity;
use game::Game;
use map::Map;
use map_version::MapVersion;
use player::Player;
use universe::Universe;
use universe_version::UniverseVersion;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    /// Current version of this API.
    fn version() -> i32 {
        1
    }

    /// Look up an account.
    async fn account(id: Uuid) -> Account {
        Account::new(id)
    }

    /// Look up a game.
    async fn game(id: Uuid) -> Game {
        Game::new(id)
    }

    /// Look up a universe.
    async fn universe(id: Uuid) -> Universe {
        Universe::new(id)
    }

    /// Search for universes.
    async fn universes(
        context: &Context,
        search: data::UniverseSearch,
    ) -> FieldResult<Pagination<Universe>> {
        let items = context
            .universes()
            .search(&search)
            .await?
            .into_iter()
            .map(|universe| Universe::new(universe.id));
        Ok(Pagination::new(search, items))
    }

    /// Search for users.
    async fn accounts(
        context: &Context,
        search: data::AccountSearch,
    ) -> FieldResult<Pagination<Account>> {
        let items = context
            .accounts()
            .search(&search)
            .await?
            .into_iter()
            .map(|account| Account::new(account.id));
        Ok(Pagination::new(search, items))
    }
}

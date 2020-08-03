use super::Context;
use juniper::FieldResult;
use uuid::Uuid;

mod traits;
pub use traits::QueryWrapper;

mod pagination;
use pagination::Pagination;

mod operation_result;
pub use operation_result::OperationResult;

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

pub use account::Account;
pub use archetype::Archetype;
pub use archetype_version::ArchetypeVersion;
pub use contributor::Contributor;
pub use email::Email;
pub use entity::Entity;
pub use game::Game;
pub use map::Map;
pub use map_version::MapVersion;
pub use player::Player;
pub use universe::Universe;
pub use universe_version::UniverseVersion;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    /// Current version of this API.
    fn version() -> i32 {
        1
    }

    /// Look up an account.
    fn account(id: Uuid) -> Account {
        Account::new(id)
    }

    /// Look up a game.
    fn game(id: Uuid) -> Game {
        Game::new(id)
    }

    /// Look up a version of a universe. If version is not specified, looks up the current (released) version.
    fn universe(context: &Context, id: Uuid, version: Option<i32>) -> FieldResult<UniverseVersion> {
        let version = match version {
            Some(version) => version,
            None => context
                .universe_versions()
                .load_current(id, false)?
                .map(|version| version.version)
                .unwrap_or(0),
        };
        Ok(UniverseVersion::new(id, version))
    }

    /// Search for universes.
    fn universes(
        context: &Context,
        search: Option<data::UniverseSearch>,
    ) -> FieldResult<Pagination<Universe>> {
        let search = search.unwrap_or_default();
        let items = context
            .universes()
            .search(&search)?
            .into_iter()
            .map(|universe| Universe::new(universe.id));
        Ok(Pagination::new(search, items))
    }

    /// Search for users.
    fn accounts(
        context: &Context,
        search: Option<data::AccountSearch>,
    ) -> FieldResult<Pagination<Account>> {
        let search = search.unwrap_or_default();
        let items = context
            .accounts()
            .search(&search)?
            .into_iter()
            .map(|account| Account::new(account.id));
        Ok(Pagination::new(search, items))
    }
}

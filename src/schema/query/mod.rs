use super::Context;
use super::loader::UniverseSearch;
use uuid::Uuid;

mod account;
mod universe;

pub use account::Account;
pub use universe::Universe;

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
            return universes.into_iter().map(Universe::new).map(Some).collect()
        }
        context.universes()
            .search(search)
            .await
            .into_iter()
            .map(|universe| Universe::new(universe.id))
            .map(Some)
            .collect()
    }

    /// User accounts.
    fn accounts(accounts: Vec<Uuid>) -> Vec<Option<Account>> {
        accounts.into_iter().map(Account::new).map(Some).collect()
    }
}

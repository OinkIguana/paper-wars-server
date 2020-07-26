use super::Context;
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
    fn universes(universes: Vec<Uuid>) -> Vec<Option<Universe>> {
        universes.into_iter().map(Universe::new).map(Some).collect()
    }

    /// User accounts.
    fn accounts(accounts: Vec<Uuid>) -> Vec<Option<Account>> {
        accounts.into_iter().map(Account::new).map(Some).collect()
    }
}

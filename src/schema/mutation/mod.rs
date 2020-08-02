use super::{query::*, Context};
use juniper::FieldResult;

mod account;
mod auth;
mod universe;

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    // -- Authentication --

    /// Attempt to sign in to the API.
    fn authenticate(
        &self,
        context: &Context,
        credentials: auth::Credentials,
    ) -> FieldResult<String> {
        self.authenticate(context, credentials)
    }

    /// When already signed in, renew the auth token to extend its expiry.
    fn renew_authentication(&self, context: &Context) -> FieldResult<Option<String>> {
        self.renew_authentication(context)
    }

    // -- Accounts --

    /// Create a new account.
    fn create_account(
        &self,
        context: &Context,
        account: account::CreateAccount,
    ) -> FieldResult<Account> {
        self.create_account(context, account)
    }

    // -- Universes --

    /// Create a new universe.
    fn create_universe(
        &self,
        context: &Context,
        universe: universe::CreateUniverse,
    ) -> FieldResult<UniverseVersion> {
        self.create_universe(context, universe)
    }

    fn invite_contributor(
        &self,
        context: &Context,
        contributor: universe::InviteContributor,
    ) -> FieldResult<Contributor> {
        self.invite_contributor(context, contributor)
    }
}

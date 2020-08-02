use super::{query::*, Context};
use juniper::FieldResult;

mod helpers;

mod account;
mod auth;
mod contributor;
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

    fn release_universe_version(
        &self,
        context: &Context,
        universe: universe::ReleaseUniverseVersion,
    ) -> FieldResult<UniverseVersion> {
        self.release_universe_version(context, universe)
    }

    // -- Contributors --

    /// Invite a person to be a contributor to a universe you own.
    fn invite_contributor(
        &self,
        context: &Context,
        contributor: contributor::InviteContributor,
    ) -> FieldResult<Contributor> {
        self.invite_contributor(context, contributor)
    }

    /// Accept or reject an invitation to contribute to a universe.
    fn respond_to_contributor_invitation(
        &self,
        context: &Context,
        invitation: contributor::RespondToContributorInvitation,
    ) -> FieldResult<Contributor> {
        self.respond_to_contributor_invitation(context, invitation)
    }
}

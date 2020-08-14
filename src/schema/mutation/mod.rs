use super::{query::*, Context};
use juniper::FieldResult;

mod helpers;

mod account;
mod archetype;
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
        credentials: Option<auth::Credentials>,
        token: Option<String>,
    ) -> FieldResult<String> {
        self.authenticate(context, credentials, token)
    }

    // -- Accounts --

    /// Create a new account.
    fn create_account(
        &self,
        context: &Context,
        account: account::CreateAccount,
    ) -> OperationResult<Account> {
        self.create_account(context, account).into()
    }

    // -- Universes --

    /// Create a new universe.
    fn create_universe(
        &self,
        context: &Context,
        universe: universe::CreateUniverse,
    ) -> OperationResult<UniverseVersion> {
        self.create_universe(context, universe).into()
    }

    /// Update the archetypes or maps included in the universe.
    fn update_universe(
        &self,
        context: &Context,
        universe: universe::UpdateUniverse,
    ) -> OperationResult<UniverseVersion> {
        self.update_universe(context, universe).into()
    }

    /// Release the current unreleased version of a universe. If no unreleased version exists, this will fail.
    fn publish_universe(
        &self,
        context: &Context,
        universe: universe::PublishUniverse,
    ) -> OperationResult<UniverseVersion> {
        self.publish_universe(context, universe).into()
    }

    // -- Contributors --

    /// Invite a person to be a contributor to a universe you own.
    fn invite_contributor(
        &self,
        context: &Context,
        contributor: contributor::InviteContributor,
    ) -> OperationResult<Contributor> {
        self.invite_contributor(context, contributor).into()
    }

    /// Accept or reject an invitation to contribute to a universe.
    fn respond_to_contributor_invitation(
        &self,
        context: &Context,
        invitation: contributor::RespondToContributorInvitation,
    ) -> OperationResult<Contributor> {
        self.respond_to_contributor_invitation(context, invitation)
            .into()
    }

    // -- Archetypes --

    /// Create a new archetype.
    fn create_archetype(
        &self,
        context: &Context,
        archetype: archetype::CreateArchetype,
    ) -> OperationResult<ArchetypeVersion> {
        self.create_archetype(context, archetype).into()
    }

    /// Update an existing archetype.
    fn update_archetype(
        &self,
        context: &Context,
        archetype: archetype::UpdateArchetype,
    ) -> OperationResult<ArchetypeVersion> {
        self.update_archetype(context, archetype).into()
    }
}

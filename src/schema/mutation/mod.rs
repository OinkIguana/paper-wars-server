use super::{query::*, Context};
use juniper::FieldResult;

mod helpers;

mod account;
mod archetype;
mod auth;
mod contributor;
mod email;
mod game;
mod map;
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

    /// Update an existing account.
    fn update_account(
        &self,
        context: &Context,
        account: account::UpdateAccount,
    ) -> OperationResult<Account> {
        self.update_account(context, account).into()
    }

    /// Add an email to the account.
    fn add_email(&self, context: &Context, email: email::AddEmail) -> OperationResult<Email> {
        self.add_email(context, email).into()
    }

    /// Remove an email from the account.
    fn remove_email(&self, context: &Context, email: email::RemoveEmail) -> OperationResult<bool> {
        self.remove_email(context, email).map(|()| true).into()
    }

    /// Verify an email address.
    fn verify_email(&self, context: &Context, email: email::VerifyEmail) -> OperationResult<Email> {
        self.verify_email(context, email).into()
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

    /// Accept an invitation to contribute to a universe.
    fn accept_contributor_invitation(
        &self,
        context: &Context,
        invitation: contributor::ContributorInvitation,
    ) -> OperationResult<Contributor> {
        self.respond_to_contributor_invitation(context, invitation, true)
            .into()
    }

    /// Reject an invitation to contribute to a universe.
    fn reject_contributor_invitation(
        &self,
        context: &Context,
        invitation: contributor::ContributorInvitation,
    ) -> OperationResult<Contributor> {
        self.respond_to_contributor_invitation(context, invitation, false)
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

    // -- Maps --

    /// Create a new map.
    fn create_map(&self, context: &Context, map: map::CreateMap) -> OperationResult<MapVersion> {
        self.create_map(context, map).into()
    }

    /// Update an existing map.
    fn update_map(&self, context: &Context, map: map::UpdateMap) -> OperationResult<MapVersion> {
        self.update_map(context, map).into()
    }

    // -- Games --

    /// Create a new game and invite players to it.
    fn create_game(&self, context: &Context, game: game::CreateGame) -> OperationResult<Game> {
        self.create_game(context, game).into()
    }

    /// Accept an invitation to a game.
    fn accept_game_invitation(
        &self,
        context: &Context,
        game: game::GameInvitation,
    ) -> OperationResult<Game> {
        self.respond_to_game_invitation(context, game, true).into()
    }

    /// Reject an invitation to a game.
    fn reject_game_invitation(
        &self,
        context: &Context,
        game: game::GameInvitation,
    ) -> OperationResult<Game> {
        self.respond_to_game_invitation(context, game, false).into()
    }
}

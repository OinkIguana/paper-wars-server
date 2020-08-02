use super::{Context, Contributor, Mutation, UniverseVersion};
use anyhow::anyhow;
use data::{contributors, universe_versions, universes, ContributorRole};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_citext::prelude::*;
use juniper::FieldResult;
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct CreateUniverse {
    name: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct InviteContributor {
    account_id: Uuid,
    universe_id: Uuid,
}

impl Mutation {
    pub(super) fn create_universe(
        &self,
        context: &Context,
        universe: CreateUniverse,
    ) -> FieldResult<UniverseVersion> {
        let account_id = context.try_authenticated_account()?;
        let (universe, contributor, universe_version) = context.transaction(|conn| {
            let name = CiString::from(universe.name.as_str());
            let universe_exists =
                select(exists(universes::table.filter(universes::name.eq(&name))))
                    .get_result(conn)?;
            if universe_exists {
                return Err(anyhow!(
                    "A universe with this name ({}) already exists",
                    &universe.name,
                ));
            }

            let universe: data::Universe = insert_into(universes::table)
                .values(universes::name.eq(&name))
                .returning(universes::all_columns)
                .get_result(conn)?;
            let contributor: data::Contributor = insert_into(contributors::table)
                .values((
                    contributors::universe_id.eq(universe.id),
                    contributors::account_id.eq(account_id),
                    contributors::role.eq(ContributorRole::Owner),
                ))
                .returning(contributors::all_columns)
                .get_result(conn)?;
            let universe_version: data::UniverseVersion = insert_into(universe_versions::table)
                .values((
                    universe_versions::universe_id.eq(universe.id),
                    universe_versions::version.eq(0),
                ))
                .returning(universe_versions::all_columns)
                .get_result(conn)?;
            Ok((universe, contributor, universe_version))
        })?;

        let query = UniverseVersion::new(universe_version.universe_id, universe_version.version);
        context.universes().prime(universe);
        context.contributors().prime(contributor);
        context.universe_versions().prime(universe_version);
        Ok(query)
    }

    pub(super) fn invite_contributor(
        &self,
        context: &Context,
        contributor: InviteContributor,
    ) -> FieldResult<Contributor> {
        let account_id = context.try_authenticated_account()?;
        let invitation = context.transaction(|conn| {
            let is_universe_owner = context
                .contributors()
                .load((contributor.universe_id, account_id))
                .map(|relationship| relationship.role == ContributorRole::Owner)
                .unwrap_or(false);
            if !is_universe_owner {
                return Err(anyhow!(
                    "You ({}) are not the owner of this universe ({})",
                    account_id,
                    contributor.universe_id,
                ));
            }
            let existing_contributor = contributors::table
                .filter(contributors::account_id.eq(contributor.account_id))
                .filter(contributors::universe_id.eq(contributor.universe_id))
                .filter(contributors::role.ne(ContributorRole::Declined));
            let contributor_exists = select(exists(existing_contributor)).get_result(conn)?;
            if contributor_exists {
                return Err(anyhow!(
                    "That account ({}) is already a contributor to this universe ({})",
                    contributor.account_id,
                    contributor.universe_id,
                ));
            }

            let invitation: data::Contributor = insert_into(contributors::table)
                .values((
                    contributors::universe_id.eq(contributor.universe_id),
                    contributors::account_id.eq(contributor.account_id),
                    contributors::role.eq(ContributorRole::Pending),
                ))
                .returning(contributors::all_columns)
                .get_result(conn)?;
            Ok(invitation)
        })?;
        let query = Contributor::new(invitation.universe_id, invitation.account_id);
        context.contributors().prime(invitation);
        Ok(query)
    }
}

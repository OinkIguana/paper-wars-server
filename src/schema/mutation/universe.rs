use super::{UniverseVersion, Context, Mutation};
use anyhow::anyhow;
use data::{universes, universe_versions, contributors};
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
            let universe_exists = select(exists(universes::table.filter(universes::name.eq(&name))))
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
                    contributors::role.eq(data::ContributorRole::Owner),
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
}

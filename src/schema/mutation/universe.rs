use super::{Context, Mutation, Universe, UniverseVersion};
use anyhow::anyhow;
use data::{
    contributors, universe_version_archetypes, universe_version_maps, universe_versions, universes,
    ContributorRole,
};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_citext::prelude::*;
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct CreateUniverse {
    name: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct CreateUniverseVersion {
    id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ReleaseUniverseVersion {
    id: Uuid,
    version: i32,
}

impl Mutation {
    pub(super) fn create_universe(
        &self,
        context: &Context,
        universe: CreateUniverse,
    ) -> anyhow::Result<Universe> {
        let account_id = context.try_authenticated_account()?;
        let (universe, contributor) = context.transaction(|conn| {
            let name = CiString::from(universe.name.as_str());
            let universe_exists: bool =
                select(exists(universes::table.filter(universes::name.eq(&name))))
                    .get_result(conn)?;
            anyhow::ensure!(
                !universe_exists,
                "A universe with this name ({}) already exists",
                &universe.name,
            );

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
            Ok((universe, contributor))
        })?;

        let query = Universe::new(universe.id);
        context.universes().prime(universe);
        context.contributors().prime(contributor);
        Ok(query)
    }

    pub(super) fn create_universe_version(
        &self,
        context: &Context,
        universe: CreateUniverseVersion,
    ) -> anyhow::Result<UniverseVersion> {
        let account_id = context.try_authenticated_account()?;
        let universe_version = context.transaction(|conn| {
            self.assert_universe_owner(context, universe.id, account_id)?;
            let unreleased_version = universe_versions::table
                .filter(universe_versions::universe_id.eq(universe.id))
                .filter(universe_versions::released_at.is_null());
            let unreleased_version_exists: bool =
                select(exists(unreleased_version)).get_result(conn)?;
            anyhow::ensure!(
                !unreleased_version_exists,
                "There is already an unreleased version of this universe ({})",
                universe.id,
            );

            let versions = universe_versions::table
                .select(count(universe_versions::star))
                .filter(universe_versions::universe_id.eq(universe.id))
                .get_result::<i64>(conn)? as i32;
            let universe_version: data::UniverseVersion = insert_into(universe_versions::table)
                .values((
                    universe_versions::universe_id.eq(universe.id),
                    universe_versions::version.eq(versions),
                ))
                .returning(universe_versions::all_columns)
                .get_result(conn)?;
            universe_version_maps::table
                .select((
                    universe_version_maps::universe_id,
                    versions.into_sql::<diesel::sql_types::Integer>(),
                    universe_version_maps::map_id,
                    universe_version_maps::map_version,
                ))
                .filter(universe_version_maps::universe_id.eq(universe.id))
                .filter(universe_version_maps::universe_version.eq(versions - 1))
                .insert_into(universe_version_maps::table)
                .execute(conn)?;
            universe_version_archetypes::table
                .select((
                    universe_version_archetypes::universe_id,
                    versions.into_sql::<diesel::sql_types::Integer>(),
                    universe_version_archetypes::archetype_id,
                    universe_version_archetypes::archetype_version,
                ))
                .filter(universe_version_archetypes::universe_id.eq(universe.id))
                .filter(universe_version_archetypes::universe_version.eq(versions - 1))
                .insert_into(universe_version_archetypes::table)
                .execute(conn)?;
            Ok(universe_version)
        })?;

        let query = UniverseVersion::new(universe_version.universe_id, universe_version.version);
        context.universe_versions().prime(universe_version);
        Ok(query)
    }

    pub(super) fn release_universe_version(
        &self,
        context: &Context,
        universe: ReleaseUniverseVersion,
    ) -> anyhow::Result<UniverseVersion> {
        let account_id = context.try_authenticated_account()?;
        let universe_version = context.transaction(|conn| {
            self.assert_universe_owner(context, universe.id, account_id)?;
            let mut universe_version = context
                .universe_versions()
                .load((universe.id, universe.version))
                .ok_or_else(|| {
                    anyhow!(
                        "Universe {} version {} does not exist, and cannot be released",
                        universe.id,
                        universe.version,
                    )
                })?;
            anyhow::ensure!(
                universe_version.released_at.is_none(),
                "Universe {} version {} has already been released",
                universe.id,
                universe.version,
            );
            universe_version.released_at = update(&universe_version)
                .set(universe_versions::released_at.eq(now))
                .returning(universe_versions::released_at)
                .get_result(conn)?;
            Ok(universe_version)
        })?;
        let query = UniverseVersion::new(universe_version.universe_id, universe_version.version);
        context.universe_versions().prime(universe_version);
        Ok(query)
    }
}

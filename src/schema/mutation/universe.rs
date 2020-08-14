use super::{Context, Mutation, UniverseVersion};
use data::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_citext::prelude::*;
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct CreateUniverse {
    name: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateUniverse {
    id: Uuid,
    add_archetypes: Option<Vec<Uuid>>,
    remove_archetypes: Option<Vec<Uuid>>,
    add_maps: Option<Vec<Uuid>>,
    remove_maps: Option<Vec<Uuid>>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct PublishUniverse {
    id: Uuid,
}

impl Mutation {
    pub(super) fn create_universe(
        &self,
        context: &Context,
        CreateUniverse { name }: CreateUniverse,
    ) -> anyhow::Result<UniverseVersion> {
        let account_id = context.try_authenticated_account()?;
        let (universe, universe_version, contributor) = context.transaction(|conn| {
            let name = CiString::from(name.as_str());
            let universe_exists: bool =
                select(exists(universes::table.filter(universes::name.eq(&name))))
                    .get_result(conn)?;
            anyhow::ensure!(
                !universe_exists,
                "A universe with this name ({}) already exists",
                &name,
            );
            let universe: data::Universe = insert_into(universes::table)
                .values(universes::name.eq(&name))
                .returning(universes::all_columns)
                .get_result(conn)?;
            let universe_version: data::UniverseVersion = insert_into(universe_versions::table)
                .values((
                    universe_versions::universe_id.eq(universe.id),
                    universe_versions::version.eq(0),
                ))
                .returning(universe_versions::all_columns)
                .get_result(conn)?;
            let contributor: data::Contributor = insert_into(contributors::table)
                .values((
                    contributors::universe_id.eq(universe.id),
                    contributors::account_id.eq(account_id),
                    contributors::role.eq(ContributorRole::Owner),
                ))
                .returning(contributors::all_columns)
                .get_result(conn)?;
            Ok((universe, universe_version, contributor))
        })?;

        let query = UniverseVersion::new(universe_version.universe_id, universe_version.version);
        context.universes().prime(universe);
        context.universe_versions().prime(universe_version);
        context.contributors().prime(contributor);
        Ok(query)
    }

    #[rustfmt::skip]
    pub(super) fn update_universe(
        &self,
        context: &Context,
        UpdateUniverse {
            id,
            add_archetypes,
            remove_archetypes,
            add_maps,
            remove_maps,
        }: UpdateUniverse,
    ) -> anyhow::Result<UniverseVersion> {
        let account_id = context.try_authenticated_account()?;
        let universe_version = context.transaction(|conn| {
            self.assert_universe_contributor(id, account_id, conn)?;
            let universe_version = self.unreleased_universe_version(id, conn)?;
            if let Some(add_archetypes) = add_archetypes {
                for archetype_id in add_archetypes {
                    let archetype_version = self.archetype_current_version(archetype_id, conn)?;
                    insert_into(universe_version_archetypes::table)
                        .values((
                            universe_version_archetypes::universe_id.eq(universe_version.universe_id),
                            universe_version_archetypes::universe_version.eq(universe_version.version),
                            universe_version_archetypes::archetype_id.eq(archetype_id),
                            universe_version_archetypes::archetype_version.eq(archetype_version),
                        ))
                        .on_conflict((
                            universe_version_archetypes::universe_id,
                            universe_version_archetypes::universe_version,
                            universe_version_archetypes::archetype_id,
                        ))
                        .do_update()
                        .set(universe_version_archetypes::archetype_version.eq(archetype_version))
                        .execute(conn)?;
                }
            }
            if let Some(remove_archetypes) = remove_archetypes {
                for archetype_id in remove_archetypes {
                    let to_delete = universe_version_archetypes::table
                        .filter(universe_version_archetypes::universe_id.eq(universe_version.universe_id))
                        .filter(universe_version_archetypes::universe_version.eq(universe_version.version))
                        .filter(universe_version_archetypes::archetype_id.eq(archetype_id));
                    delete(to_delete).execute(conn)?;
                }
            }
            if let Some(add_maps) = add_maps {
                for map_id in add_maps {
                    let map_version = self.map_current_version(map_id, conn)?;
                    insert_into(universe_version_maps::table)
                        .values((
                            universe_version_maps::universe_id.eq(universe_version.universe_id),
                            universe_version_maps::universe_version.eq(universe_version.version),
                            universe_version_maps::map_id.eq(map_id),
                            universe_version_maps::map_version.eq(map_version),
                        ))
                        .on_conflict((
                            universe_version_maps::universe_id,
                            universe_version_maps::universe_version,
                            universe_version_maps::map_id,
                        ))
                        .do_update()
                        .set(universe_version_maps::map_version.eq(map_version))
                        .execute(conn)?;
                }
            }
            if let Some(remove_maps) = remove_maps {
                for map_id in remove_maps {
                    let to_delete = universe_version_maps::table
                        .filter(universe_version_maps::universe_id.eq(universe_version.universe_id))
                        .filter(universe_version_maps::universe_version.eq(universe_version.version))
                        .filter(universe_version_maps::map_id.eq(map_id));
                    delete(to_delete).execute(conn)?;
                }
            }
            Ok(universe_version)
        })?;

        let query = UniverseVersion::new(universe_version.universe_id, universe_version.version);
        context.universe_versions().prime(universe_version);
        Ok(query)
    }

    pub(super) fn publish_universe(
        &self,
        context: &Context,
        PublishUniverse { id }: PublishUniverse,
    ) -> anyhow::Result<UniverseVersion> {
        let account_id = context.try_authenticated_account()?;
        let universe_version = context.transaction(|conn| {
            self.assert_universe_owner(id, account_id, conn)?;
            let mut universe_version: data::UniverseVersion = universe_versions::table
                .filter(universe_versions::universe_id.eq(id))
                .filter(universe_versions::released_at.is_null())
                .get_result(conn)?;
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

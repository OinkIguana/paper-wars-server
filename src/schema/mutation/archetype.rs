use super::{ArchetypeVersion, Context, Mutation};
use data::*;
use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct CreateArchetype {
    name: String,
    universe: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateArchetype {
    id: Uuid,
    script: String,
}

impl Mutation {
    pub(super) fn create_archetype(
        &self,
        context: &Context,
        CreateArchetype { name, universe }: CreateArchetype,
    ) -> anyhow::Result<ArchetypeVersion> {
        let account_id = context.try_authenticated_account()?;
        let (archetype, archetype_version) = context.transaction(|conn| {
            self.assert_universe_contributor(universe, account_id, conn)?;
            let existing = archetypes::table
                .filter(archetypes::universe_id.eq(&universe))
                .filter(archetypes::name.eq(&name));
            let archetype_exists: bool = select(exists(existing)).get_result(conn)?;
            anyhow::ensure!(
                !archetype_exists,
                "An archetype with this name ({}) already exists",
                &name,
            );
            let archetype: data::Archetype = insert_into(archetypes::table)
                .values((
                    archetypes::name.eq(&name),
                    archetypes::universe_id.eq(&universe),
                ))
                .returning(archetypes::all_columns)
                .get_result(conn)?;
            let archetype_version: data::ArchetypeVersion = insert_into(archetype_versions::table)
                .values((
                    archetype_versions::archetype_id.eq(archetype.id),
                    archetype_versions::version.eq(0),
                    archetype_versions::script.eq(""),
                ))
                .returning(archetype_versions::all_columns)
                .get_result(conn)?;
            Ok((archetype, archetype_version))
        })?;

        let query =
            ArchetypeVersion::new(archetype_version.archetype_id, archetype_version.version);
        context.archetypes().prime(archetype);
        context.archetype_versions().prime(archetype_version);
        Ok(query)
    }

    #[rustfmt::skip]
    pub(super) fn update_archetype(
        &self,
        context: &Context,
        UpdateArchetype { id, script }: UpdateArchetype,
    ) -> anyhow::Result<ArchetypeVersion> {
        let account_id = context.try_authenticated_account()?;
        let archetype_version: data::ArchetypeVersion = context.transaction(|conn| {
            let archetype = archetypes::table
                .filter(archetypes::id.eq(id))
                .get_result::<Archetype>(conn)?;
            self.assert_universe_contributor(archetype.universe_id, account_id, conn)?;
            let most_recent_version = self.archetype_current_version(archetype.id, conn)?;
            let same_universe_version = universe_versions::universe_id.eq(universe_version_archetypes::universe_id)
                .and(universe_versions::version.eq(universe_version_archetypes::universe_version));
            let version_in_use = universe_version_archetypes::table
                .inner_join(universe_versions::table.on(same_universe_version))
                .filter(universe_version_archetypes::archetype_id.eq(archetype.id))
                .filter(universe_version_archetypes::archetype_version.eq(most_recent_version))
                .filter(universe_versions::released_at.is_not_null());
            let version_in_use = select(exists(version_in_use)).get_result(conn)?;
            let archetype_version: data::ArchetypeVersion = if version_in_use {
                insert_into(archetype_versions::table)
                    .values((
                        archetype_versions::archetype_id.eq(archetype.id),
                        archetype_versions::version.eq(most_recent_version + 1),
                        archetype_versions::script.eq(&script),
                    ))
                    .returning(archetype_versions::all_columns)
                    .get_result(conn)?
            } else {
                update(archetype_versions::table)
                    .set(archetype_versions::script.eq(&script))
                    .filter(archetype_versions::archetype_id.eq(id))
                    .filter(archetype_versions::version.eq(most_recent_version))
                    .returning(archetype_versions::all_columns)
                    .get_result(conn)?
            };
            let universe_version = self.unreleased_universe_version(archetype.universe_id, conn)?;
            update(universe_version_archetypes::table)
                .filter(universe_version_archetypes::universe_id.eq(universe_version.universe_id))
                .filter(universe_version_archetypes::universe_version.eq(universe_version.version))
                .filter(universe_version_archetypes::archetype_id.eq(archetype_version.archetype_id))
                .set(universe_version_archetypes::archetype_version.eq(archetype_version.version))
                .execute(conn)?;
            Ok(archetype_version)
        })?;

        let query =
            ArchetypeVersion::new(archetype_version.archetype_id, archetype_version.version);
        context.archetype_versions().prime(archetype_version);
        Ok(query)
    }
}

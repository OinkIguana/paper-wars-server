use super::{Archetype, ArchetypeVersion, Context, Mutation};
use anyhow::anyhow;
use data::{archetype_versions, archetypes};
use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct CreateArchetype {
    name: String,
    universe: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct CreateArchetypeVersion {
    id: Uuid,
}

impl Mutation {
    pub(super) fn create_archetype(
        &self,
        context: &Context,
        archetype: CreateArchetype,
    ) -> anyhow::Result<Archetype> {
        let account_id = context.try_authenticated_account()?;
        let archetype = context.transaction(|conn| {
            self.assert_universe_contributor(context, archetype.universe, account_id)?;
            let existing = archetypes::table
                .filter(archetypes::universe_id.eq(&archetype.universe))
                .filter(archetypes::name.eq(&archetype.name));
            let archetype_exists: bool = select(exists(existing)).get_result(conn)?;
            anyhow::ensure!(
                !archetype_exists,
                "An archetype with this name ({}) already exists",
                &archetype.name,
            );

            let archetype: data::Archetype = insert_into(archetypes::table)
                .values((
                    archetypes::name.eq(&archetype.name),
                    archetypes::universe_id.eq(&archetype.universe),
                ))
                .returning(archetypes::all_columns)
                .get_result(conn)?;
            Ok(archetype)
        })?;

        let query = Archetype::new(archetype.id);
        context.archetypes().prime(archetype);
        Ok(query)
    }

    pub(super) fn create_archetype_version(
        &self,
        context: &Context,
        archetype: CreateArchetypeVersion,
    ) -> anyhow::Result<ArchetypeVersion> {
        let account_id = context.try_authenticated_account()?;
        let version = context.transaction(|conn| {
            let archetype = context
                .archetypes()
                .load(archetype.id)
                .ok_or_else(|| anyhow!("No archetype {} was found", archetype.id))?;
            self.assert_universe_contributor(context, archetype.universe_id, account_id)?;
            let versions = archetype_versions::table
                .select(count(archetype_versions::star))
                .filter(archetype_versions::archetype_id.eq(archetype.id))
                .get_result::<i64>(conn)? as i32;
            let script = if versions == 0 {
                String::new()
            } else {
                archetype_versions::table
                    .select(archetype_versions::script)
                    .filter(archetype_versions::archetype_id.eq(archetype.id))
                    .filter(archetype_versions::version.eq(versions - 1))
                    .get_result::<String>(conn)?
            };
            let version = insert_into(archetype_versions::table)
                .values((
                    archetype_versions::archetype_id.eq(archetype.id),
                    archetype_versions::version.eq(versions),
                    archetype_versions::script.eq(script),
                ))
                .returning(archetype_versions::all_columns)
                .get_result::<data::ArchetypeVersion>(conn)?;
            Ok(version)
        })?;

        let query = ArchetypeVersion::new(version.archetype_id, version.version);
        context.archetype_versions().prime(version);
        Ok(query)
    }
}

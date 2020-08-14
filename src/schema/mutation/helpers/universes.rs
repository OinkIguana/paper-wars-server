use super::Mutation;
use data::*;
use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

impl Mutation {
    pub fn unreleased_universe_version(
        &self,
        universe_id: Uuid,
        conn: &DbConnection,
    ) -> anyhow::Result<UniverseVersion> {
        universe_versions::table
            .filter(universe_versions::universe_id.eq(universe_id))
            .filter(universe_versions::released_at.is_null())
            .get_result(conn)
            .or_else(|_| {
                let current_version = universe_versions::table
                    .select(max(universe_versions::version))
                    .filter(universe_versions::universe_id.eq(universe_id))
                    .get_result::<Option<i32>>(conn)?
                    .unwrap_or(0);
                let universe_version = insert_into(universe_versions::table)
                    .values((
                        universe_versions::universe_id.eq(universe_id),
                        universe_versions::version.eq(current_version + 1),
                    ))
                    .returning(universe_versions::all_columns)
                    .get_result(conn)?;
                universe_version_maps::table
                    .select((
                        universe_version_maps::universe_id,
                        (current_version + 1).into_sql::<diesel::sql_types::Integer>(),
                        universe_version_maps::map_id,
                        universe_version_maps::map_version,
                    ))
                    .filter(universe_version_maps::universe_id.eq(universe_id))
                    .filter(universe_version_maps::universe_version.eq(current_version))
                    .insert_into(universe_version_maps::table)
                    .execute(conn)?;
                universe_version_archetypes::table
                    .select((
                        universe_version_archetypes::universe_id,
                        (current_version + 1).into_sql::<diesel::sql_types::Integer>(),
                        universe_version_archetypes::archetype_id,
                        universe_version_archetypes::archetype_version,
                    ))
                    .filter(universe_version_archetypes::universe_id.eq(universe_id))
                    .filter(universe_version_archetypes::universe_version.eq(current_version))
                    .insert_into(universe_version_archetypes::table)
                    .execute(conn)?;
                Ok(universe_version)
            })
    }
}

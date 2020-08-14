use super::{Context, MapVersion, Mutation};
use data::*;
use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct CreateMap {
    name: String,
    universe: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct UpdateMap {
    id: Uuid,
    script: String,
}

impl Mutation {
    pub(super) fn create_map(
        &self,
        context: &Context,
        CreateMap { name, universe }: CreateMap,
    ) -> anyhow::Result<MapVersion> {
        let account_id = context.try_authenticated_account()?;
        let (map, map_version) = context.transaction(|conn| {
            self.assert_universe_contributor(universe, account_id, conn)?;
            let existing = maps::table
                .filter(maps::universe_id.eq(&universe))
                .filter(maps::name.eq(&name));
            let map_exists: bool = select(exists(existing)).get_result(conn)?;
            anyhow::ensure!(
                !map_exists,
                "An map with this name ({}) already exists",
                &name,
            );
            let map: data::Map = insert_into(maps::table)
                .values((maps::name.eq(&name), maps::universe_id.eq(&universe)))
                .returning(maps::all_columns)
                .get_result(conn)?;
            let map_version: data::MapVersion = insert_into(map_versions::table)
                .values((
                    map_versions::map_id.eq(map.id),
                    map_versions::version.eq(0),
                    map_versions::script.eq(""),
                ))
                .returning(map_versions::all_columns)
                .get_result(conn)?;
            Ok((map, map_version))
        })?;

        let query = MapVersion::new(map_version.map_id, map_version.version);
        context.maps().prime(map);
        context.map_versions().prime(map_version);
        Ok(query)
    }

    #[rustfmt::skip]
    pub(super) fn update_map(
        &self,
        context: &Context,
        UpdateMap { id, script }: UpdateMap,
    ) -> anyhow::Result<MapVersion> {
        let account_id = context.try_authenticated_account()?;
        let map_version: data::MapVersion = context.transaction(|conn| {
            let map = maps::table
                .filter(maps::id.eq(id))
                .get_result::<Map>(conn)?;
            self.assert_universe_contributor(map.universe_id, account_id, conn)?;
            let most_recent_version = self.map_current_version(map.id, conn)?;
            let same_universe_version = universe_versions::universe_id.eq(universe_version_maps::universe_id)
                .and(universe_versions::version.eq(universe_version_maps::universe_version));
            let version_in_use = universe_version_maps::table
                .inner_join(universe_versions::table.on(same_universe_version))
                .filter(universe_version_maps::map_id.eq(map.id))
                .filter(universe_version_maps::map_version.eq(most_recent_version))
                .filter(universe_versions::released_at.is_not_null());
            let version_in_use = select(exists(version_in_use)).get_result(conn)?;
            let map_version: data::MapVersion = if version_in_use {
                insert_into(map_versions::table)
                    .values((
                        map_versions::map_id.eq(map.id),
                        map_versions::version.eq(most_recent_version + 1),
                        map_versions::script.eq(&script),
                    ))
                    .returning(map_versions::all_columns)
                    .get_result(conn)?
            } else {
                update(map_versions::table)
                    .set(map_versions::script.eq(&script))
                    .filter(map_versions::map_id.eq(id))
                    .filter(map_versions::version.eq(most_recent_version))
                    .returning(map_versions::all_columns)
                    .get_result(conn)?
            };
            let universe_version = self.unreleased_universe_version(map.universe_id, conn)?;
            update(universe_version_maps::table)
                .filter(universe_version_maps::universe_id.eq(universe_version.universe_id))
                .filter(universe_version_maps::universe_version.eq(universe_version.version))
                .filter(universe_version_maps::map_id.eq(map_version.map_id))
                .set(universe_version_maps::map_version.eq(map_version.version))
                .execute(conn)?;
            Ok(map_version)
        })?;

        let query =
            MapVersion::new(map_version.map_id, map_version.version);
        context.map_versions().prime(map_version);
        Ok(query)
    }
}

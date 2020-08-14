use super::Mutation;
use data::*;
use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

impl Mutation {
    pub fn map_current_version(&self, map_id: Uuid, conn: &DbConnection) -> anyhow::Result<i32> {
        Ok(map_versions::table
            .select(max(map_versions::version))
            .filter(map_versions::map_id.eq(map_id))
            .get_result::<Option<i32>>(conn)?
            .unwrap())
    }
}

use super::Mutation;
use data::*;
use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

impl Mutation {
    pub fn archetype_current_version(
        &self,
        archetype_id: Uuid,
        conn: &DbConnection,
    ) -> anyhow::Result<i32> {
        Ok(archetype_versions::table
            .select(max(archetype_versions::version))
            .filter(archetype_versions::archetype_id.eq(archetype_id))
            .get_result::<Option<i32>>(conn)?
            .unwrap())
    }
}

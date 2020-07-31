use super::{traits::BatchFnItem, Loader};
use data::{universe_versions, UniverseVersion};
use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

batch_fn!(universe_versions => UniverseVersion { universe_id: Uuid, version: i32 });

impl Loader<(Uuid, i32), UniverseVersion> {
    join!(universe_versions => for_universe(universe_id: Uuid) -> UniverseVersion);

    pub async fn load_current(
        &self,
        universe_id: Uuid,
        unreleased: bool,
    ) -> anyhow::Result<Option<UniverseVersion>> {
        let version: Option<UniverseVersion> =
            tokio::task::block_in_place(|| -> anyhow::Result<Option<UniverseVersion>> {
                let conn = self.database.connection()?;

                let mut max_version = universe_versions::table
                    .select(max(universe_versions::version))
                    .filter(universe_versions::universe_id.eq(universe_id))
                    .into_boxed();
                if !unreleased {
                    max_version = max_version.filter(universe_versions::released_at.is_not_null());
                }
                match max_version.get_result::<Option<i32>>(&conn)? {
                    Some(max_version) => Ok(universe_versions::table
                        .filter(universe_versions::universe_id.eq(universe_id))
                        .filter(universe_versions::version.eq(max_version))
                        .get_result::<UniverseVersion>(&conn)
                        .optional()?),
                    None => Ok(None),
                }
            })?;

        if let Some(version) = &version {
            self.prime(version.key(), Some(version.clone())).await;
        }
        Ok(version)
    }
}

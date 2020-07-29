use super::Loader;
use data::UniverseVersion;
use uuid::Uuid;

batch_fn!(universe_versions => UniverseVersion { universe_id: Uuid, version: i32 });

impl Loader<(Uuid, i32), UniverseVersion> {
    join!(universe_versions => for_universe(universe_id: Uuid) -> UniverseVersion);
}

use super::Loader;
use data::UniverseVersionMap;
use uuid::Uuid;

batch_fn!(universe_version_maps => UniverseVersionMap { universe_id: Uuid, universe_version: i32, map_id: Uuid });

impl Loader<(Uuid, i32, Uuid), UniverseVersionMap> {
    join!(universe_version_maps => for_universe_version(universe_id: Uuid, universe_version: i32) -> UniverseVersionMap);
}

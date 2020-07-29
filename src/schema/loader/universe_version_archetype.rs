use super::Loader;
use data::UniverseVersionArchetype;
use uuid::Uuid;

batch_fn!(universe_version_archetypes => UniverseVersionArchetype { universe_id: Uuid, universe_version: i32, archetype_id: Uuid });

impl Loader<(Uuid, i32, Uuid), UniverseVersionArchetype> {
    join!(universe_version_archetypes => for_universe_version(universe_id: Uuid, universe_version: i32) -> UniverseVersionArchetype);
}

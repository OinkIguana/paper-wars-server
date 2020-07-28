use super::Loader;
use data::ArchetypeVersion;
use uuid::Uuid;

batch_fn!(archetype_versions => ArchetypeVersion { archetype_id: Uuid, version: i32 });

impl Loader<(Uuid, i32), ArchetypeVersion> {
    join!(archetype_versions => for_archetype(archetype_id: Uuid) -> ArchetypeVersion);
}

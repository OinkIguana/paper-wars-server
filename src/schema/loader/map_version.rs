use super::Loader;
use data::MapVersion;
use uuid::Uuid;

batch_fn!(map_versions => MapVersion { map_id: Uuid, version: i32 });

impl Loader<(Uuid, i32), MapVersion> {
    join!(map_versions => for_map(map_id: Uuid) -> MapVersion);
}

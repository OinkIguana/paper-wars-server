use super::Loader;
use data::Map;
use uuid::Uuid;

batch_fn!(maps => Map { id: Uuid });

impl Loader<Uuid, Map> {
    join!(maps => for_universe(universe_id: Uuid) -> Map);
}

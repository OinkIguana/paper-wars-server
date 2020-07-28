use super::Loader;
use data::Archetype;
use uuid::Uuid;

batch_fn!(archetypes => Archetype { id: Uuid });

impl Loader<Uuid, Archetype> {
    join!(archetypes => for_universe(universe_id: Uuid) -> Archetype);
}

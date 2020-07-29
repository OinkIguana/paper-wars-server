use super::Loader;
use data::Entity;
use uuid::Uuid;

batch_fn!(entities => Entity { id: Uuid });

impl Loader<Uuid, Entity> {
    join!(entities => for_game(game_id: Uuid) -> Entity);
}

use super::{ArchetypeVersion, Context, Player, QueryWrapper};
use anyhow::anyhow;
use juniper::FieldResult;
use uuid::Uuid;

pub struct Entity {
    id: Uuid,
}

impl QueryWrapper for Entity {
    type Model = data::Entity;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .entities()
            .load(self.id)
            .ok_or_else(|| anyhow!("Entity {} does not exist", self.id))
    }
}

impl Entity {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    fn load_game(&self, context: &Context, entity: &data::Entity) -> anyhow::Result<data::Game> {
        context
            .games()
            .load(entity.game_id)
            .ok_or_else(|| anyhow!("Game {} does not exist", entity.game_id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl Entity {
    /// The ID of the entity.
    fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context)?.id)
    }

    /// The archetype of this entity.
    fn archetype(&self, context: &Context) -> FieldResult<ArchetypeVersion> {
        let entity = self.load(context)?;
        let game = self.load_game(context, &entity)?;
        let version = context
            .universe_version_archetypes()
            .load((game.universe_id, game.universe_version, entity.archetype_id))
            .ok_or_else(|| {
                anyhow!(
                    "Universe {} version {} archetype {} does not exist",
                    game.universe_id,
                    game.universe_version,
                    entity.archetype_id
                )
            })?;
        Ok(ArchetypeVersion::new(
            version.archetype_id,
            version.archetype_version,
        ))
    }

    /// The owner of this entity.
    fn player(&self, context: &Context) -> FieldResult<Option<Player>> {
        let entity = self.load(context)?;
        Ok(entity
            .account_id
            .map(|account_id| Player::new(entity.game_id, account_id)))
    }

    /// The game state that is specific to this entity.
    fn state(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context)?.state.to_string())
    }
}

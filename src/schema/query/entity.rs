use super::{ArchetypeVersion, Context, Player, QueryWrapper};
use anyhow::anyhow;
use juniper::FieldResult;
use uuid::Uuid;

pub struct Entity {
    id: Uuid,
}

#[async_trait::async_trait]
impl QueryWrapper for Entity {
    type Model = data::Entity;

    async fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .entities()
            .load(self.id)
            .await
            .ok_or_else(|| anyhow!("Entity {} does not exist", self.id))
    }
}

impl Entity {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    async fn load_game(
        &self,
        context: &Context,
        entity: &data::Entity,
    ) -> anyhow::Result<data::Game> {
        context
            .games()
            .load(entity.game_id)
            .await
            .ok_or_else(|| anyhow!("Game {} does not exist", entity.game_id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl Entity {
    /// The ID of the entity.
    async fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context).await?.id)
    }

    /// The archetype of this entity.
    async fn archetype(&self, context: &Context) -> FieldResult<ArchetypeVersion> {
        let entity = self.load(context).await?;
        let game = self.load_game(context, &entity).await?;
        let version = context
            .universe_version_archetypes()
            .load((game.universe_id, game.universe_version, entity.archetype_id))
            .await
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
    async fn player(&self, context: &Context) -> FieldResult<Option<Player>> {
        let entity = self.load(context).await?;
        Ok(entity
            .account_id
            .map(|account_id| Player::new(entity.game_id, account_id)))
    }

    /// The game state that is specific to this entity.
    async fn state(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.state.to_string())
    }
}

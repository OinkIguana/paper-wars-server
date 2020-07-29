use super::{Context, UniverseVersion, MapVersion, Player, Entity};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct Game {
    id: Uuid,
}

impl Game {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    async fn load(&self, context: &Context) -> anyhow::Result<data::Game> {
        context
            .games()
            .load(self.id)
            .await
            .ok_or_else(|| anyhow!("Game {} does not exist", self.id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl Game {
    /// The ID of the game.
    async fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context).await?.id)
    }

    /// The name of the game, chosen by the "host" to identify it.
    async fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.name.to_owned())
    }

    /// The universe this game takes place in.
    async fn universe(&self, context: &Context) -> FieldResult<UniverseVersion> {
        let game = self.load(context).await?;
        let universe = context
            .universe_versions()
            .load((game.universe_id, game.universe_version))
            .await
            .ok_or_else(|| anyhow!("Universe {} version {} does not exist", game.universe_id, game.universe_version))?;
        Ok(UniverseVersion::new(universe.universe_id, universe.version))
    }

    /// The map this game takes place on.
    async fn map(&self, context: &Context) -> FieldResult<MapVersion> {
        let game = self.load(context).await?;
        let map = context
            .universe_version_maps()
            .load((game.universe_id, game.universe_version, game.map_id))
            .await
            .ok_or_else(|| anyhow!("Universe {} version {} map {} does not exist", game.universe_id, game.universe_version, game.map_id))?;
        Ok(MapVersion::new(map.map_id, map.map_version))
    }

    /// The seed provided to the map script to generate the random aspects of the game.
    async fn map_seed(&self, context: &Context) -> FieldResult<String> {
        Ok(hex::encode(self.load(context).await?.map_seed))
    }

    /// The state of the game.
    async fn state(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.state.to_string())
    }

    /// When this game was started.
    async fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.created_at)
    }

    /// The players in this game.
    async fn players(&self, context: &Context) -> FieldResult<Vec<Player>> {
        Ok(context
            .players()
            .for_game(&self.load(context).await?.id)
            .await
            .into_iter()
            .map(|player| Player::new(player.game_id, player.account_id))
            .collect())
    }

    /// The entities that currently exist in game.
    async fn entities(&self, context: &Context) -> FieldResult<Vec<Entity>> {
        Ok(context
            .entities()
            .for_game(&self.load(context).await?.id)
            .await
            .into_iter()
            .map(|entity| Entity::new(entity.id))
            .collect())
    }
}

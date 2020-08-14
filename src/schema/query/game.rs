use super::{
    Context, Entity, MapVersion, OperationResult, Pagination, Player, QueryWrapper, UniverseVersion,
};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct Game {
    id: Uuid,
}

impl QueryWrapper for Game {
    type Model = data::Game;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .games()
            .load(self.id)
            .ok_or_else(|| anyhow!("Game {} does not exist", self.id))
    }
}

impl Game {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[juniper::graphql_object(Context = Context)]
impl Game {
    /// The ID of the game.
    fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context)?.id)
    }

    /// The name of the game, chosen by the "host" to identify it.
    fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context)?.name.to_owned())
    }

    /// The universe this game takes place in.
    fn universe(&self, context: &Context) -> FieldResult<UniverseVersion> {
        let game = self.load(context)?;
        let universe = context
            .universe_versions()
            .load((game.universe_id, game.universe_version))
            .ok_or_else(|| {
                anyhow!(
                    "Universe {} version {} does not exist",
                    game.universe_id,
                    game.universe_version
                )
            })?;
        Ok(UniverseVersion::new(universe.universe_id, universe.version))
    }

    /// The map this game takes place on.
    fn map(&self, context: &Context) -> FieldResult<MapVersion> {
        let game = self.load(context)?;
        let map = context
            .universe_version_maps()
            .load((game.universe_id, game.universe_version, game.map_id))
            .ok_or_else(|| {
                anyhow!(
                    "Universe {} version {} map {} does not exist",
                    game.universe_id,
                    game.universe_version,
                    game.map_id
                )
            })?;
        Ok(MapVersion::new(map.map_id, map.map_version))
    }

    /// The seed provided to the map script to generate the random aspects of the game.
    fn map_seed(&self, context: &Context) -> FieldResult<String> {
        Ok(base64::encode(self.load(context)?.map_seed))
    }

    /// The state of the game.
    fn state(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context)?.state.to_string())
    }

    /// When this game was started.
    fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context)?.created_at)
    }

    /// The players in this game.
    fn players(&self, context: &Context) -> FieldResult<Vec<Player>> {
        Ok(context
            .players()
            .for_game(&self.load(context)?.id)
            .into_iter()
            .map(|player| Player::new(player.game_id, player.account_id))
            .collect())
    }

    /// The entities that currently exist in game.
    fn entities(&self, context: &Context) -> FieldResult<Vec<Entity>> {
        Ok(context
            .entities()
            .for_game(&self.load(context)?.id)
            .into_iter()
            .map(|entity| Entity::new(entity.id))
            .collect())
    }
}

#[juniper::graphql_object(Context = Context, name = "GamePagination")]
impl Pagination<Game> {
    fn items(&self) -> &[Game] {
        self.items()
    }

    fn total(&self) -> i32 {
        self.total()
    }

    fn start(&self, context: &Context) -> juniper::FieldResult<Option<String>> {
        self.start(context)
    }

    fn end(&self, context: &Context) -> juniper::FieldResult<Option<String>> {
        self.end(context)
    }
}

#[juniper::graphql_object(Context = Context, name = "GameResult")]
impl OperationResult<Game> {
    pub fn success(&self) -> Option<&Game> {
        self.success()
    }

    pub fn error(&self) -> Option<String> {
        self.error()
    }
}

use super::{Context, MapVersion};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct Map {
    id: Uuid,
}

impl Map {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    async fn load(&self, context: &Context) -> anyhow::Result<data::Map> {
        context
            .maps()
            .load(self.id)
            .await
            .ok_or_else(|| anyhow!("Map {} does not exist", self.id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl Map {
    /// The ID of the map.
    async fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context).await?.id)
    }

    /// The development name of the map. This should not be used in game.
    async fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.name.to_owned())
    }

    /// When this map was created.
    async fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.created_at)
    }

    /// Versions of this map.
    async fn versions(&self, context: &Context) -> FieldResult<Vec<MapVersion>> {
        Ok(context
            .map_versions()
            .for_map(&self.load(context).await?.id)
            .await
            .into_iter()
            .map(|version| MapVersion::new(version.map_id, version.version))
            .collect())
    }
}

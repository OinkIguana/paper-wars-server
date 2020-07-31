use super::Context;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct MapVersion {
    map_id: Uuid,
    version: i32,
}

impl MapVersion {
    pub fn new(map_id: Uuid, version: i32) -> Self {
        Self { map_id, version }
    }

    async fn load_map(&self, context: &Context) -> anyhow::Result<data::Map> {
        context
            .maps()
            .load(self.map_id)
            .await
            .ok_or_else(|| anyhow!("Map {} does not exist", self.map_id))
    }

    async fn load(&self, context: &Context) -> anyhow::Result<data::MapVersion> {
        context
            .map_versions()
            .load((self.map_id, self.version))
            .await
            .ok_or_else(|| {
                anyhow!(
                    "Map {} version {} does not exist",
                    self.map_id,
                    self.version
                )
            })
    }
}

#[juniper::graphql_object(Context = Context)]
impl MapVersion {
    /// The ID of the map.
    async fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load_map(context).await?.id)
    }

    /// The development name of the map. This should not be used in game.
    async fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load_map(context).await?.name.to_owned())
    }

    /// The version number.
    async fn version(&self, context: &Context) -> FieldResult<i32> {
        Ok(self.load(context).await?.version)
    }

    /// The script defining the behaviour and attributes of this archetype.
    async fn script(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.script.to_owned())
    }

    /// When this version was created.
    async fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.created_at)
    }
}

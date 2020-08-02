use super::{Context, QueryWrapper};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct MapVersion {
    map_id: Uuid,
    version: i32,
}

impl QueryWrapper for MapVersion {
    type Model = data::MapVersion;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .map_versions()
            .load((self.map_id, self.version))
            .ok_or_else(|| {
                anyhow!(
                    "Map {} version {} does not exist",
                    self.map_id,
                    self.version
                )
            })
    }
}

impl MapVersion {
    pub fn new(map_id: Uuid, version: i32) -> Self {
        Self { map_id, version }
    }

    fn load_map(&self, context: &Context) -> anyhow::Result<data::Map> {
        context
            .maps()
            .load(self.map_id)
            .ok_or_else(|| anyhow!("Map {} does not exist", self.map_id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl MapVersion {
    /// The ID of the map.
    fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load_map(context)?.id)
    }

    /// The development name of the map. This should not be used in game.
    fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load_map(context)?.name.to_owned())
    }

    /// The version number.
    fn version(&self, context: &Context) -> FieldResult<i32> {
        Ok(self.load(context)?.version)
    }

    /// The script defining the behaviour and attributes of this archetype.
    fn script(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context)?.script.to_owned())
    }

    /// When this version was created.
    fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context)?.created_at)
    }
}

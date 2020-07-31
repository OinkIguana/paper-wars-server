use super::Context;
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct ArchetypeVersion {
    archetype_id: Uuid,
    version: i32,
}

impl ArchetypeVersion {
    pub fn new(archetype_id: Uuid, version: i32) -> Self {
        Self {
            archetype_id,
            version,
        }
    }

    async fn load_archetype(&self, context: &Context) -> anyhow::Result<data::Archetype> {
        context
            .archetypes()
            .load(self.archetype_id)
            .await
            .ok_or_else(|| anyhow!("Archetype {} does not exist", self.archetype_id))
    }

    async fn load(&self, context: &Context) -> anyhow::Result<data::ArchetypeVersion> {
        context
            .archetype_versions()
            .load((self.archetype_id, self.version))
            .await
            .ok_or_else(|| {
                anyhow!(
                    "Archetype {} version {} does not exist",
                    self.archetype_id,
                    self.version
                )
            })
    }
}

#[juniper::graphql_object(Context = Context)]
impl ArchetypeVersion {
    /// The ID of the archetype.
    async fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load_archetype(context).await?.id)
    }

    /// The development name of the archetype. This should not be used in game.
    async fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load_archetype(context).await?.name.to_owned())
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

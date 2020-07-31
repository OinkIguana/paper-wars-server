use super::{ArchetypeVersion, Context};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct Archetype {
    id: Uuid,
}

impl Archetype {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    async fn load(&self, context: &Context) -> anyhow::Result<data::Archetype> {
        context
            .archetypes()
            .load(self.id)
            .await
            .ok_or_else(|| anyhow!("Archetype {} does not exist", self.id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl Archetype {
    /// The ID of the archetype.
    async fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context).await?.id)
    }

    /// The development name of the archetype. This should not be used in game.
    async fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.name.to_owned())
    }

    /// When this archetype was created.
    async fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.created_at)
    }

    async fn versions(&self, context: &Context) -> FieldResult<Vec<ArchetypeVersion>> {
        Ok(context
            .archetype_versions()
            .for_archetype(&self.load(context).await?.id)
            .await
            .into_iter()
            .map(|version| ArchetypeVersion::new(version.archetype_id, version.version))
            .collect())
    }
}

use super::{Context, ArchetypeVersion, MapVersion};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct UniverseVersion {
    universe_id: Uuid,
    version: i32,
}

impl UniverseVersion {
    pub fn new(universe_id: Uuid, version: i32) -> Self {
        Self { universe_id, version }
    }

    async fn load_universe(&self, context: &Context) -> anyhow::Result<data::Universe> {
        context
            .universes()
            .load(self.universe_id)
            .await
            .ok_or_else(|| anyhow!("Universe {} does not exist", self.universe_id))
    }

    async fn load(&self, context: &Context) -> anyhow::Result<data::UniverseVersion> {
        context
            .universe_versions()
            .load((self.universe_id, self.version))
            .await
            .ok_or_else(|| anyhow!("Universe {} version {} does not exist", self.universe_id, self.version))
    }
}

#[juniper::graphql_object(Context = Context)]
impl UniverseVersion {
    /// The ID of the universe.
    async fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load_universe(context).await?.id)
    }

    /// The name of the universe. This should be compared case-insensitively.
    async fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load_universe(context).await?.name.to_string())
    }

    /// The version number.
    async fn version(&self, context: &Context) -> FieldResult<i32> {
        Ok(self.load(context).await?.version)
    }

    /// When this version was created.
    async fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.created_at)
    }

    /// When this version was released. If null, this version is unreleased.
    async fn released_at(&self, context: &Context) -> FieldResult<Option<DateTime<Utc>>> {
        Ok(self.load(context).await?.released_at)
    }

    /// Archetypes available in this version.
    async fn archetypes(&self, context: &Context) -> FieldResult<Vec<ArchetypeVersion>> {
        let universe = self.load(context).await?;
        Ok(context
            .universe_version_archetypes()
            .for_universe_version(&universe.universe_id, &universe.version)
            .await
            .into_iter()
            .map(|version| ArchetypeVersion::new(version.archetype_id, version.archetype_version))
            .collect())
    }

    /// Maps available in this version.
    async fn maps(&self, context: &Context) -> FieldResult<Vec<MapVersion>> {
        let universe = self.load(context).await?;
        Ok(context
            .universe_version_maps()
            .for_universe_version(&universe.universe_id, &universe.version)
            .await
            .into_iter()
            .map(|version| MapVersion::new(version.map_id, version.map_version))
            .collect())
    }
}

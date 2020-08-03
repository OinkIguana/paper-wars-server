use super::{ArchetypeVersion, Context, MapVersion, OperationResult, QueryWrapper};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct UniverseVersion {
    universe_id: Uuid,
    version: i32,
}

impl QueryWrapper for UniverseVersion {
    type Model = data::UniverseVersion;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .universe_versions()
            .load((self.universe_id, self.version))
            .ok_or_else(|| {
                anyhow!(
                    "Universe {} version {} does not exist",
                    self.universe_id,
                    self.version
                )
            })
    }
}

impl UniverseVersion {
    pub fn new(universe_id: Uuid, version: i32) -> Self {
        Self {
            universe_id,
            version,
        }
    }

    fn load_universe(&self, context: &Context) -> anyhow::Result<data::Universe> {
        context
            .universes()
            .load(self.universe_id)
            .ok_or_else(|| anyhow!("Universe {} does not exist", self.universe_id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl UniverseVersion {
    /// The ID of the universe.
    fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load_universe(context)?.id)
    }

    /// The name of the universe. This should be compared case-insensitively.
    fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load_universe(context)?.name.to_string())
    }

    /// The version number.
    fn version(&self, context: &Context) -> FieldResult<i32> {
        Ok(self.load(context)?.version)
    }

    /// When this version was created.
    fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context)?.created_at)
    }

    /// When this version was released. If null, this version is unreleased.
    fn released_at(&self, context: &Context) -> FieldResult<Option<DateTime<Utc>>> {
        Ok(self.load(context)?.released_at)
    }

    /// Archetypes available in this version.
    fn archetypes(&self, context: &Context) -> FieldResult<Vec<ArchetypeVersion>> {
        let universe = self.load(context)?;
        Ok(context
            .universe_version_archetypes()
            .for_universe_version(&universe.universe_id, &universe.version)
            .into_iter()
            .map(|version| ArchetypeVersion::new(version.archetype_id, version.archetype_version))
            .collect())
    }

    /// Maps available in this version.
    fn maps(&self, context: &Context) -> FieldResult<Vec<MapVersion>> {
        let universe = self.load(context)?;
        Ok(context
            .universe_version_maps()
            .for_universe_version(&universe.universe_id, &universe.version)
            .into_iter()
            .map(|version| MapVersion::new(version.map_id, version.map_version))
            .collect())
    }
}

#[juniper::graphql_object(Context = Context, name = "UniverseVersionResult")]
impl OperationResult<UniverseVersion> {
    pub fn success(&self) -> Option<&UniverseVersion> {
        self.success()
    }

    pub fn error(&self) -> Option<String> {
        self.error()
    }
}

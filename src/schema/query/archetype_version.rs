use super::{Context, QueryWrapper};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct ArchetypeVersion {
    archetype_id: Uuid,
    version: i32,
}

impl QueryWrapper for ArchetypeVersion {
    type Model = data::ArchetypeVersion;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .archetype_versions()
            .load((self.archetype_id, self.version))
            .ok_or_else(|| {
                anyhow!(
                    "Archetype {} version {} does not exist",
                    self.archetype_id,
                    self.version
                )
            })
    }
}

impl ArchetypeVersion {
    pub fn new(archetype_id: Uuid, version: i32) -> Self {
        Self {
            archetype_id,
            version,
        }
    }

    fn load_archetype(&self, context: &Context) -> anyhow::Result<data::Archetype> {
        context
            .archetypes()
            .load(self.archetype_id)
            .ok_or_else(|| anyhow!("Archetype {} does not exist", self.archetype_id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl ArchetypeVersion {
    /// The ID of the archetype.
    fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load_archetype(context)?.id)
    }

    /// The development name of the archetype. This should not be used in game.
    fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load_archetype(context)?.name.to_owned())
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

use super::{ArchetypeVersion, Context, OperationResult, QueryWrapper};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct Archetype {
    id: Uuid,
}

impl QueryWrapper for Archetype {
    type Model = data::Archetype;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .archetypes()
            .load(self.id)
            .ok_or_else(|| anyhow!("Archetype {} does not exist", self.id))
    }
}

impl Archetype {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[juniper::graphql_object(Context = Context)]
impl Archetype {
    /// The ID of the archetype.
    fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context)?.id)
    }

    /// The development name of the archetype. This should not be used in game.
    fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context)?.name.to_owned())
    }

    /// When this archetype was created.
    fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context)?.created_at)
    }

    fn versions(&self, context: &Context) -> FieldResult<Vec<ArchetypeVersion>> {
        Ok(context
            .archetype_versions()
            .for_archetype(&self.load(context)?.id)
            .into_iter()
            .map(|version| ArchetypeVersion::new(version.archetype_id, version.version))
            .collect())
    }
}

#[juniper::graphql_object(Context = Context, name = "ArchetypeResult")]
impl OperationResult<Archetype> {
    pub fn success(&self) -> Option<&Archetype> {
        self.success()
    }

    pub fn error(&self) -> Option<String> {
        self.error()
    }
}

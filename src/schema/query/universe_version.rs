use super::Context;
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
}

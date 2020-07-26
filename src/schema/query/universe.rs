use anyhow::anyhow;
use super::Context;
use juniper::FieldResult;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Universe {
    id: Uuid,
}

impl Universe {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    async fn load(&self, context: &Context) -> anyhow::Result<data::Universe> {
        context
            .universes()
            .load(self.id)
            .await
            .ok_or_else(|| anyhow!("Universe {} does not exist", self.id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl Universe {
    /// The ID of the universe.
    async fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context).await?.id)
    }

    /// The name of the universe. This should be compared case-insensitively.
    async fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.name.to_string())
    }

    /// When this universe was created.
    async fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.created_at)
    }
}

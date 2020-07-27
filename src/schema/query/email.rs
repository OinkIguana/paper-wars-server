use anyhow::anyhow;
use super::Context;
use diesel_citext::types::CiString;
use juniper::FieldResult;
use chrono::{DateTime, Utc};

pub struct Email {
    address: CiString,
}

impl Email {
    pub fn new(address: impl Into<CiString>) -> Self {
        Self { address: address.into() }
    }

    async fn load(&self, context: &Context) -> anyhow::Result<data::Email> {
        context
            .emails()
            .load(self.address.to_owned())
            .await
            .ok_or_else(|| anyhow!("Email {} does not exist", self.address))
    }
}

#[juniper::graphql_object(Context = Context)]
impl Email {
    async fn address(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.address.into())
    }

    async fn verified_at(&self, context: &Context) -> FieldResult<Option<DateTime<Utc>>> {
        Ok(self.load(context).await?.verified_at)
    }

    async fn protected_until(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.protected_until)
    }

    /// When this email was created.
    async fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.created_at)
    }
}

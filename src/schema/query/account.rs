use anyhow::anyhow;
use super::{Context, Email};
use juniper::FieldResult;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Account {
    id: Uuid,
}

impl Account {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    async fn load(&self, context: &Context) -> anyhow::Result<data::Account> {
        context
            .accounts()
            .load(self.id)
            .await
            .ok_or_else(|| anyhow!("Account {} does not exist", self.id))
    }
}

#[juniper::graphql_object(Context = Context)]
impl Account {
    /// The ID of the account.
    async fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context).await?.id)
    }

    /// The username of the account. This should be compared case-insensitively.
    async fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.name.to_string())
    }

    /// When this account was created.
    async fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.created_at)
    }

    /// Email addresses associated with this account. This should only be viewable to the
    /// account's owner.
    async fn emails(&self, context: &Context) -> Vec<Email> {
        context.emails()
            .for_account(&self.id)
            .await
            .into_iter()
            .map(|email| Email::new(email.address))
            .collect()
    }
}

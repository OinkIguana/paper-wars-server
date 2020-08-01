use super::{Context, QueryWrapper};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use diesel_citext::types::CiString;
use juniper::FieldResult;

pub struct Email {
    address: CiString,
}

#[async_trait::async_trait]
impl QueryWrapper for Email {
    type Model = data::Email;

    async fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .emails()
            .load(self.address.to_owned())
            .await
            .ok_or_else(|| anyhow!("Email {} does not exist", self.address))
    }
}

impl Email {
    pub fn new(address: impl Into<CiString>) -> Self {
        Self {
            address: address.into(),
        }
    }
}

#[juniper::graphql_object(Context = Context)]
impl Email {
    /// The actual email address.
    async fn address(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context).await?.address.into())
    }

    /// Whether this email has been verified.
    async fn verified_at(&self, context: &Context) -> FieldResult<Option<DateTime<Utc>>> {
        Ok(self.load(context).await?.verified_at)
    }

    /// How long this email is protected while unverified.
    async fn protected_until(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.protected_until)
    }

    /// When this email was created.
    async fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context).await?.created_at)
    }

    /// Whether this is the primary (login and contact) email address for this account.
    async fn is_primary_email(&self, context: &Context) -> FieldResult<bool> {
        let account_id = self.load(context).await?.account_id;
        Ok(context
            .logins()
            .load(account_id)
            .await
            .map(|login| login.email_address == self.address)
            .unwrap_or(false))
    }
}

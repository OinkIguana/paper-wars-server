use super::{Context, OperationResult, QueryWrapper};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use diesel_citext::types::CiString;
use juniper::FieldResult;

pub struct Email {
    address: CiString,
}

impl QueryWrapper for Email {
    type Model = data::Email;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .emails()
            .load(self.address.to_owned())
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
    fn address(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context)?.address.into())
    }

    /// Whether this email has been verified.
    fn verified_at(&self, context: &Context) -> FieldResult<Option<DateTime<Utc>>> {
        Ok(self.load(context)?.verified_at)
    }

    /// How long this email is protected while unverified.
    fn protected_until(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context)?.protected_until)
    }

    /// When this email was created.
    fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context)?.created_at)
    }

    /// Whether this is the primary (login and contact) email address for this account.
    fn is_primary_email(&self, context: &Context) -> FieldResult<bool> {
        let account_id = self.load(context)?.account_id;
        Ok(context
            .logins()
            .load(account_id)
            .map(|login| login.email_address == self.address)
            .unwrap_or(false))
    }
}

#[juniper::graphql_object(Context = Context, name = "EmailResult")]
impl OperationResult<Email> {
    pub fn success(&self) -> Option<&Email> {
        self.success()
    }

    pub fn error(&self) -> Option<String> {
        self.error()
    }
}

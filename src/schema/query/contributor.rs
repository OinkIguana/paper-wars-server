use super::{Account, Context, OperationResult, Pagination, QueryWrapper, Universe};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use data::ContributorRole;
use juniper::FieldResult;
use uuid::Uuid;

pub struct Contributor {
    universe_id: Uuid,
    account_id: Uuid,
}

impl QueryWrapper for Contributor {
    type Model = data::Contributor;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .contributors()
            .load((self.universe_id, self.account_id))
            .ok_or_else(|| {
                anyhow!(
                    "Contributor {} to {} does not exist",
                    self.account_id,
                    self.universe_id
                )
            })
    }
}

impl Contributor {
    pub fn new(universe_id: Uuid, account_id: Uuid) -> Self {
        Self {
            universe_id,
            account_id,
        }
    }
}

#[juniper::graphql_object(Context = Context)]
impl Contributor {
    /// The account that is contributing.
    async fn account(&self, context: &Context) -> FieldResult<Account> {
        Ok(Account::new(self.load(context)?.account_id))
    }

    /// The universe they are contributing to.
    async fn universe(&self, context: &Context) -> FieldResult<Universe> {
        Ok(Universe::new(self.load(context)?.universe_id))
    }

    /// The role this account has in the contribution.
    async fn role(&self, context: &Context) -> FieldResult<ContributorRole> {
        Ok(self.load(context)?.role)
    }

    /// When they started contributing to this universe.
    async fn contributor_since(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context)?.created_at)
    }
}

#[juniper::graphql_object(Context = Context, name = "ContributorPagination")]
impl Pagination<Contributor> {
    fn items(&self) -> &[Contributor] {
        self.items()
    }

    fn total(&self) -> i32 {
        self.total()
    }

    fn start(&self, context: &Context) -> juniper::FieldResult<Option<String>> {
        self.start(context)
    }

    fn end(&self, context: &Context) -> juniper::FieldResult<Option<String>> {
        self.end(context)
    }
}

#[juniper::graphql_object(Context = Context, name = "ContributorResult")]
impl OperationResult<Contributor> {
    pub fn success(&self) -> Option<&Contributor> {
        self.success()
    }

    pub fn error(&self) -> Option<String> {
        self.error()
    }
}

use super::{Context, Contributor, Email, Game, Pagination, QueryWrapper};
use anyhow::anyhow;
use chrono::{DateTime, Utc};
use juniper::FieldResult;
use uuid::Uuid;

pub struct Account {
    id: Uuid,
}

impl QueryWrapper for Account {
    type Model = data::Account;

    fn load(&self, context: &Context) -> anyhow::Result<Self::Model> {
        context
            .accounts()
            .load(self.id)
            .ok_or_else(|| anyhow!("Account {} does not exist", self.id))
    }
}

impl Account {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[juniper::graphql_object(Context = Context)]
impl Account {
    /// The ID of the account.
    fn id(&self, context: &Context) -> FieldResult<Uuid> {
        Ok(self.load(context)?.id)
    }

    /// The username of the account. This should be compared case-insensitively.
    fn name(&self, context: &Context) -> FieldResult<String> {
        Ok(self.load(context)?.name.to_string())
    }

    /// When this account was created.
    fn created_at(&self, context: &Context) -> FieldResult<DateTime<Utc>> {
        Ok(self.load(context)?.created_at)
    }

    /// Email addresses associated with this account. This should only be viewable to the
    /// account's owner.
    fn emails(&self, context: &Context) -> FieldResult<Vec<Email>> {
        Ok(context
            .emails()
            .for_account(&self.load(context)?.id)
            .into_iter()
            .map(|email| Email::new(email.address))
            .collect())
    }

    /// The universes that this account is a contributor to.
    fn contributions(
        &self,
        context: &Context,
        search: Option<data::ContributorSearch>,
    ) -> FieldResult<Pagination<Contributor>> {
        let search = search.unwrap_or_default().for_account(self.id);
        let items = context
            .contributors()
            .search(&search)?
            .into_iter()
            .map(|contributor| Contributor::new(contributor.universe_id, contributor.account_id));
        Ok(Pagination::new(search, items))
    }

    /// Games that this person is playing.
    fn games(&self, context: &Context) -> FieldResult<Vec<Game>> {
        Ok(context
            .players()
            .for_account(&self.load(context)?.id)
            .into_iter()
            .map(|player| Game::new(player.game_id))
            .collect())
    }
}

#[juniper::graphql_object(Context = Context, name = "AccountPagination")]
impl Pagination<Account> {
    fn items(&self) -> &[Account] {
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

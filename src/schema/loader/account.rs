use super::Loader;
use data::{accounts, emails, Account};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_citext::prelude::*;
use tokio::task;
use uuid::Uuid;

batch_fn!(accounts => Account { id: Uuid });

impl Loader<Uuid, Account> {
    pub async fn search(&self, search: Option<AccountSearch>) -> Vec<Account> {
        let load_result: anyhow::Result<Vec<Account>> = task::block_in_place(|| {
            let conn = self.database.connection()?;
            let mut query = accounts::table.into_boxed();
            if let Some(search) = search {
                if let Some(search_name) = search.name {
                    query = query.filter(accounts::name.like(CiString::from(format!("%{}%", search_name))));
                }
                if let Some(search_email) = search.email {
                    query = query.filter(exists(
                        emails::table
                            .filter(emails::address.eq(CiString::from(search_email)))
                            .filter(emails::account_id.eq(accounts::id)),
                    ));
                }
                if let Some(limit) = search.limit {
                    query = query.limit(limit as i64);
                }
                if let Some(offset) = search.cursor {
                    query = query.offset(offset as i64);
                }
            }
            Ok(query.load(&conn)?)
        });
        let items = load_result.unwrap_or(vec![]);
        let to_cache = items
            .iter()
            .cloned()
            .map(|item| (item.id.to_owned(), Some(item)));
        self.prime_many(to_cache).await;
        items
    }
}

#[derive(juniper::GraphQLInputObject)]
pub struct AccountSearch {
    name: Option<String>,
    email: Option<String>,
    limit: Option<i32>,
    cursor: Option<i32>,
}

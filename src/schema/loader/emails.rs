use data::{emails::dsl::*, Email};
use diesel::prelude::*;
use diesel_citext::types::CiString;
use uuid::Uuid;
use super::Loader;
use tokio::task;

batch_fn!(CiString, Email, emails, address);

impl Loader<CiString, Email> {
    pub async fn for_account(&self, id: &Uuid) -> Vec<Email> {
        let load_result: anyhow::Result<Vec<Email>> = task::block_in_place(|| {
            let conn = self.database.connection()?;
            Ok(emails
                .filter(account_id.eq(id))
                .load(&conn)?)
        });

        let items = load_result.unwrap_or(vec![]);
        let to_cache = items.iter()
            .cloned()
            .map(|item| (item.address.to_owned(), Some(item)));
        self.prime_many(to_cache).await;
        items
    }
}

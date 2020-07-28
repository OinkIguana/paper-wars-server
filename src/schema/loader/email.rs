use super::Loader;
use data::{emails, Email};
use diesel::prelude::*;
use diesel_citext::types::CiString;
use tokio::task;
use uuid::Uuid;

batch_fn!(emails => Email { address: CiString });

impl Loader<CiString, Email> {
    pub async fn for_account(&self, id: &Uuid) -> Vec<Email> {
        let load_result: anyhow::Result<Vec<Email>> = task::block_in_place(|| {
            let conn = self.database.connection()?;
            Ok(emails::table
                .filter(emails::account_id.eq(id))
                .load(&conn)?)
        });

        let items = load_result.unwrap_or(vec![]);
        self.prime_many(items.clone()).await;
        items
    }
}

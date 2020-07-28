use super::Loader;
use data::{contributors, Contributor};
use diesel::prelude::*;
use tokio::task;
use uuid::Uuid;

batch_fn!(contributors => Contributor { universe_id: Uuid, account_id: Uuid });

impl Loader<(Uuid, Uuid), Contributor> {
    pub async fn for_account(&self, id: &Uuid) -> Vec<Contributor> {
        let load_result: anyhow::Result<Vec<Contributor>> = task::block_in_place(|| {
            let conn = self.database.connection()?;
            Ok(contributors::table
                .filter(contributors::account_id.eq(id))
                .load(&conn)?)
        });

        let items = load_result.unwrap_or(vec![]);
        self.prime_many(items.clone()).await;
        items
    }

    pub async fn for_universe(&self, id: &Uuid) -> Vec<Contributor> {
        let load_result: anyhow::Result<Vec<Contributor>> = task::block_in_place(|| {
            let conn = self.database.connection()?;
            Ok(contributors::table
                .filter(contributors::universe_id.eq(id))
                .load(&conn)?)
        });

        let items = load_result.unwrap_or(vec![]);
        self.prime_many(items.clone()).await;
        items
    }
}

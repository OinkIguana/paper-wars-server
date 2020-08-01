use super::Loader;
use data::{accounts, logins, Login};
use diesel::prelude::*;
use diesel_citext::prelude::*;
use uuid::Uuid;

batch_fn!(logins => Login { account_id: Uuid });

impl Loader<Uuid, Login> {
    pub async fn for_account_with_name(&self, name: &str) -> anyhow::Result<Option<Login>> {
        let login: Option<Login> =
            tokio::task::block_in_place(|| -> anyhow::Result<Option<Login>> {
                let conn = self.database.connection()?;
                Ok(logins::table
                    .select(logins::all_columns)
                    .inner_join(accounts::table)
                    .filter(accounts::name.eq(CiString::from(name)))
                    .get_result::<Login>(&conn)
                    .optional()?)
            })?;

        if let Some(login) = &login {
            self.prime(login.clone()).await;
        }
        Ok(login)
    }

    pub async fn by_email_address(&self, email: &str) -> anyhow::Result<Option<Login>> {
        let login: Option<Login> =
            tokio::task::block_in_place(|| -> anyhow::Result<Option<Login>> {
                let conn = self.database.connection()?;
                Ok(logins::table
                    .filter(logins::email_address.eq(CiString::from(email)))
                    .get_result::<Login>(&conn)
                    .optional()?)
            })?;

        if let Some(login) = &login {
            self.prime(login.clone()).await;
        }
        Ok(login)
    }
}

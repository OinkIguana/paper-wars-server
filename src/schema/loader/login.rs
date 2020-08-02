use super::Loader;
use data::{accounts, logins, Login};
use diesel::prelude::*;
use diesel_citext::prelude::*;
use uuid::Uuid;

batch_fn!(logins => Login { account_id: Uuid });

impl Loader<Uuid, Login> {
    pub fn for_account_with_name(&self, name: &str) -> anyhow::Result<Option<Login>> {
        let conn = self.database.connection()?;
        let login: Option<Login> = logins::table
            .select(logins::all_columns)
            .inner_join(accounts::table)
            .filter(accounts::name.eq(CiString::from(name)))
            .get_result::<Login>(&conn)
            .optional()?;
        if let Some(login) = &login {
            self.prime(login.clone());
        }
        Ok(login)
    }

    pub fn by_email_address(&self, email: &str) -> anyhow::Result<Option<Login>> {
        let conn = self.database.connection()?;
        let login = logins::table
            .filter(logins::email_address.eq(CiString::from(email)))
            .get_result::<Login>(&conn)
            .optional()?;
        if let Some(login) = &login {
            self.prime(login.clone());
        }
        Ok(login)
    }
}

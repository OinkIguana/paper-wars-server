use super::{Account, Context, Mutation};
use data::{accounts, emails, logins};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_citext::prelude::*;

#[derive(juniper::GraphQLInputObject)]
pub struct CreateAccount {
    name: String,
    email: String,
    password: String,
}

impl Mutation {
    pub(super) fn create_account(
        &self,
        context: &Context,
        account: CreateAccount,
    ) -> anyhow::Result<Account> {
        let (account, email, login) = context.transaction(|conn| {
            let name = CiString::from(account.name.as_str());
            let address = CiString::from(account.email.as_str());

            let is_active = emails::verified_at
                .is_not_null()
                .or(emails::protected_until.gt(now));
            let matching_email = emails::table
                .filter(emails::address.eq(&address))
                .filter(is_active);
            let email_exists: bool = select(exists(matching_email)).get_result(conn)?;
            anyhow::ensure!(
                !email_exists,
                "An account with this email ({}) already exists.",
                &account.email
            );

            let name_exists: bool =
                select(exists(accounts::table.filter(accounts::name.eq(&name))))
                    .get_result(conn)?;
            anyhow::ensure!(
                !name_exists,
                "An account with this name ({}) already exists.",
                &account.name
            );

            let hashed_password = bcrypt::hash(&account.password, bcrypt::DEFAULT_COST)?;

            let account: data::Account = insert_into(accounts::table)
                .values(accounts::name.eq(&name))
                .returning(accounts::all_columns)
                .get_result(conn)?;
            let email: data::Email = insert_into(emails::table)
                .values((
                    emails::account_id.eq(&account.id),
                    emails::address.eq(&address),
                ))
                .returning(emails::all_columns)
                .get_result(conn)?;
            let login: data::Login = insert_into(logins::table)
                .values((
                    logins::account_id.eq(&account.id),
                    logins::email_address.eq(&email.address),
                    logins::password.eq(hashed_password),
                ))
                .returning(logins::all_columns)
                .get_result(conn)?;
            Ok((account, email, login))
        })?;

        let query = Account::new(account.id);
        context.accounts().prime(account);
        context.emails().prime(email);
        context.logins().prime(login);
        Ok(query)
    }
}

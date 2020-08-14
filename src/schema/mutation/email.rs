use super::{Context, Email, Mutation};
use data::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_citext::prelude::*;
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct AddEmail {
    email: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct RemoveEmail {
    email: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct VerifyEmail {
    email: String,
    account: Uuid,
    signature: String,
}

impl Mutation {
    pub(super) fn add_email(
        &self,
        context: &Context,
        AddEmail { email }: AddEmail,
    ) -> anyhow::Result<Email> {
        let account_id = context.try_authenticated_account()?;
        let email = context.transaction(|conn| {
            let address = CiString::from(email.as_str());
            let email: data::Email = insert_into(emails::table)
                .values((
                    emails::account_id.eq(account_id),
                    emails::address.eq(&address),
                ))
                .returning(emails::all_columns)
                .get_result(conn)?;
            Ok(email)
        })?;
        let query = Email::new(email.address.clone());
        context.emails().prime(email);
        Ok(query)
    }

    pub(super) fn remove_email(
        &self,
        context: &Context,
        RemoveEmail { email }: RemoveEmail,
    ) -> anyhow::Result<()> {
        let account_id = context.try_authenticated_account()?;
        context.transaction(|conn| {
            let address = CiString::from(email.as_str());
            let matched_email = emails::table
                .filter(emails::account_id.eq(account_id))
                .filter(emails::address.eq(&address));
            delete(matched_email).execute(conn)?;
            Ok(())
        })?;
        Ok(())
    }

    pub(super) fn verify_email(
        &self,
        _context: &Context,
        VerifyEmail {
            email: _,
            account: _,
            signature: _,
        }: VerifyEmail,
    ) -> anyhow::Result<Email> {
        anyhow::bail!("We do not yet verify emails");
    }
}

use super::{query::*, Context};
use juniper::FieldResult;

mod account;
mod auth;

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    /// Attempt to sign in to the API.
    async fn authenticate(
        &self,
        context: &Context,
        credentials: auth::Credentials,
    ) -> FieldResult<String> {
        self.authenticate(context, credentials).await
    }

    /// When already signed in, renew the auth token to extend its expiry.
    async fn renew_authentication(&self, context: &Context) -> FieldResult<Option<String>> {
        self.renew_authentication(context).await
    }

    /// Create a new account.
    async fn create_account(
        &self,
        context: &Context,
        account: account::CreateAccount,
    ) -> FieldResult<Account> {
        self.create_account(context, account).await
    }
}

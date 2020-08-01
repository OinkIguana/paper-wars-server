use super::Context;
use crate::jwt;
use anyhow::anyhow;
use juniper::FieldResult;

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    /// Attempt to sign in to the API.
    async fn authenticate(
        context: &Context,
        username: Option<String>,
        email: Option<String>,
        password: String,
    ) -> FieldResult<String> {
        let login = match (username, email) {
            (None, None) | (Some(_), Some(_)) => {
                return Err(anyhow!("Exactly one of username or email must be supplied").into())
            }
            (Some(username), _) => context.logins().for_account_with_name(&username).await?,
            (_, Some(email)) => context.logins().by_email_address(&email).await?,
        };
        let login = login.ok_or_else(|| anyhow!("Account was not found"))?;
        if bcrypt::verify(password, &login.password)? {
            let account = context.accounts().load(login.account_id).await.unwrap();
            Ok(jwt::encode(account)?)
        } else {
            Err(anyhow!("Incorrect password").into())
        }
    }

    /// When already signed in, renew the auth token to extend its expiry.
    async fn renew_authentication(context: &Context) -> FieldResult<Option<String>> {
        let account_id = match context.authenticated_account() {
            Some(account_id) => account_id,
            None => return Ok(None),
        };
        let account = context
            .accounts()
            .load(account_id)
            .await
            .unwrap();
        Ok(Some(jwt::encode(account)?))
    }
}

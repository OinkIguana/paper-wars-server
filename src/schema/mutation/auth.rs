use super::{Context, Mutation};
use crate::jwt;
use anyhow::anyhow;
use juniper::FieldResult;

#[derive(juniper::GraphQLInputObject)]
pub struct Credentials {
    name: Option<String>,
    email: Option<String>,
    password: String,
}

impl Mutation {
    /// Attempt to sign in to the API.
    pub(super) async fn authenticate(
        &self,
        context: &Context,
        credentials: Credentials,
    ) -> FieldResult<String> {
        let login = match (credentials.name, credentials.email) {
            (None, None) | (Some(_), Some(_)) => {
                return Err(anyhow!("Exactly one of name or email must be supplied").into())
            }
            (Some(name), _) => context.logins().for_account_with_name(&name).await?,
            (_, Some(email)) => context.logins().by_email_address(&email).await?,
        };
        let login = login.ok_or_else(|| anyhow!("Account was not found"))?;
        if bcrypt::verify(credentials.password, &login.password)? {
            let account = context.accounts().load(login.account_id).await.unwrap();
            Ok(jwt::encode(account)?)
        } else {
            Err(anyhow!("Incorrect password").into())
        }
    }

    /// When already signed in, renew the auth token to extend its expiry.
    pub(super) async fn renew_authentication(
        &self,
        context: &Context,
    ) -> FieldResult<Option<String>> {
        let account_id = match context.authenticated_account() {
            Some(account_id) => account_id,
            None => return Ok(None),
        };
        let account = context.accounts().load(account_id).await.unwrap();
        Ok(Some(jwt::encode(account)?))
    }
}

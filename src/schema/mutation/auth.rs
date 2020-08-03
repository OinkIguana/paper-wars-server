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
    pub(super) fn authenticate(
        &self,
        context: &Context,
        credentials: Option<Credentials>,
        token: Option<String>,
    ) -> FieldResult<String> {
        let account = if let Some(credentials) = credentials {
            let login = match (credentials.name, credentials.email) {
                (None, None) | (Some(_), Some(_)) => {
                    return Err(anyhow!("Exactly one of name or email must be supplied").into())
                }
                (Some(name), _) => context.logins().for_account_with_name(&name)?,
                (_, Some(email)) => context.logins().by_email_address(&email)?,
            };
            let login = login.ok_or_else(|| anyhow!("Account was not found"))?;
            if bcrypt::verify(credentials.password, &login.password)? {
                context.accounts().load(login.account_id).unwrap()
            } else {
                return Err(anyhow!("Incorrect password").into());
            }
        } else {
            let account_id = if let Some(token) = token {
                jwt::decode(&token)?
            } else {
                context
                    .authenticated_account()
                    .ok_or(anyhow!("No credentials were supplied"))?
            };
            context
                .accounts()
                .load(account_id)
                .ok_or(anyhow!("This account no longer exists"))?
        };
        let account_id = account.id;
        let token = jwt::encode(account)?;
        context.set_authenticated_account(account_id);
        Ok(token)
    }
}

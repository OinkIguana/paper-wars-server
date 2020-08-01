use anyhow::anyhow;
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::Status;
use std::env;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    iat: usize,
    exp: usize,
}

fn secret() -> Vec<u8> {
    let secret = env::var("JWT_SECRET").unwrap();
    hex::decode(secret).unwrap()
}

pub fn encode(account: data::Account) -> anyhow::Result<String> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims {
            sub: account.id.to_string(),
            iss: String::from("paper-wars"),
            iat: Utc::now().timestamp() as usize,
            exp: Utc::now().timestamp() as usize + 60 * 60 * 24 * 7,
        },
        &EncodingKey::from_secret(&secret()),
    )?)
}

pub fn decode(jwt: &str) -> anyhow::Result<Uuid> {
    let token = jsonwebtoken::decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(&secret()),
        &Validation {
            leeway: 60,
            iss: Some(String::from("paper-wars")),
            ..Validation::default()
        },
    )?;
    Ok(Uuid::parse_str(&token.claims.sub)?)
}

#[derive(Clone, Debug)]
pub struct AuthenticatedAccount(Uuid);

impl Into<Uuid> for AuthenticatedAccount {
    fn into(self) -> Uuid {
        self.0
    }
}

#[async_trait::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedAccount {
    type Error = anyhow::Error;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let header = request
            .headers()
            .get("Authorization")
            .next();
        let token = match header {
            Some(header) if header.starts_with("Bearer") => header[6..].trim(),
            Some(..) => return Outcome::Failure((Status::Unauthorized, anyhow!("Only Bearer authorization is supported"))),
            None => return Outcome::Forward(()),
        };
        match decode(token) {
            Ok(id) => Outcome::Success(AuthenticatedAccount(id)),
            Err(error) => Outcome::Failure((Status::Unauthorized, error.into())),
        }
    }
}

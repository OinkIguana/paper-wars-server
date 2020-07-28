use super::Loader;
use data::Email;
use diesel_citext::types::CiString;
use uuid::Uuid;

batch_fn!(emails => Email { address: CiString });

impl Loader<CiString, Email> {
    join!(emails => for_account(account_id: Uuid) -> Email);
}

use super::Loader;
use data::Contributor;
use uuid::Uuid;

batch_fn!(contributors => Contributor { universe_id: Uuid, account_id: Uuid });

impl Loader<(Uuid, Uuid), Contributor> {
    join!(contributors => for_account(account_id: Uuid) -> Contributor);
    join!(contributors => for_universe(universe_id: Uuid) -> Contributor);
}

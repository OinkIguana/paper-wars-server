use data::Contributor;
use uuid::Uuid;

batch_fn!(contributors => Contributor { universe_id: Uuid, account_id: Uuid });

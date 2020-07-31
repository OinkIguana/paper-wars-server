use data::Account;
use uuid::Uuid;

batch_fn!(accounts => Account { id: Uuid });

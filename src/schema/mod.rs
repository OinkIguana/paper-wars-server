use juniper::RootNode;

mod context;
mod database;
mod loader;
mod mutation;
mod query;
mod subscription;

pub use context::Context;
pub use database::Database;
pub use loader::Loader;
pub use mutation::Mutation;
pub use query::Query;
pub use subscription::Subscription;

pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn create() -> Schema {
    Schema::new(Query, Mutation::new(), Subscription::new())
}

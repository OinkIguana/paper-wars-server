use juniper::RootNode;

mod context;
mod mutation;
mod query;

pub use context::Context;
pub use mutation::Mutation;
pub use query::Query;

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create() -> Schema {
    Schema::new(Query, Mutation)
}

use super::Context;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn version() -> i32 {
        1
    }
}

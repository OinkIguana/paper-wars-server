use super::Context;
use uuid::Uuid;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn universe(&self, context: &Context, universe: Uuid) -> Option<Universe> {
        None
    }

    fn universes(universes: Vec<Uuid>) -> Vec<Option<Universe>> {
        vec![]
    }
}

struct Universe {
    data: data::Universe,
}

#[juniper::graphql_object(Context = Context)]
impl Universe {
    fn name(&self) -> String {
        self.data.name.to_string()
    }
}

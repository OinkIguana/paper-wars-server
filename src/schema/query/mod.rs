use super::Context;

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {}

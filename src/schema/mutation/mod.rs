use super::Context;

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {}

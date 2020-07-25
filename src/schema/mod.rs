use data::Database;
use juniper::RootNode;

pub struct Context {
    database: Database,
}

impl Context {
    pub fn new(database_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            database: Database::connect(database_url)?,
        })
    }
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create() -> Schema {
    Schema::new(Query, Mutation)
}

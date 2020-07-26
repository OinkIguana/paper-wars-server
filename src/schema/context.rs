use data::Database;

pub struct Context {
    database: Database,
}

impl Context {
    pub fn new(database_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            database: Database::connect(database_url)?,
        })
    }

    pub fn database(&self) -> &Database {
        &self.database
    }
}

impl juniper::Context for Context {}

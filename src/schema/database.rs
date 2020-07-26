use data::DbConnection;

#[derive(Clone)]
pub struct Database(data::Database);

impl Database {
    pub fn connect(database_url: String) -> anyhow::Result<Self> {
        data::Database::connect(database_url).map(Self)
    }

    pub fn connection(&self) -> anyhow::Result<DbConnection> {
        self.0.connection()
    }
}

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

    /// Starts a transaction using a connection to this database. The provided function
    /// will be called with that connection.
    pub fn transaction<T, F>(&self, transaction: F) -> anyhow::Result<T>
    where
        F: FnOnce(&DbConnection) -> anyhow::Result<T>,
    {
        self.0.transaction(transaction)
    }
}

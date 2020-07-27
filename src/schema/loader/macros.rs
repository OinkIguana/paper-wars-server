#[macro_export]
macro_rules! batch_fn {
    ($model:ty, $table:ident) => {
        batch_fn!(uuid::Uuid, $model, $table);
    };

    ($key:ty, $model:ty, $table:ident) => {
        batch_fn!($key, $model, $table, id);
    };

    ($key:ty, $model:ty, $table:ident, $id:ident) => {
        #[async_trait::async_trait]
        impl dataloader::BatchFn<$key, Option<$model>> for crate::schema::Database {
            async fn load(&self, keys: &[$key]) -> std::collections::HashMap<$key, Option<$model>> {
                use diesel::prelude::*;
                let mut map: std::collections::HashMap<$key, Option<$model>> = keys.iter()
                    .cloned()
                    .map(|key| (key, None))
                    .collect();
                let load_result: anyhow::Result<Vec<$model>> = tokio::task::block_in_place(|| {
                    let conn = self.connection()?;
                    Ok(data::$table::table
                        .filter(data::$table::$id.eq_any(keys))
                        .load(&conn)?)
                });
                let items = load_result.unwrap_or(vec![]);
                for item in items {
                    map.get_mut(&item.$id).unwrap().replace(item);
                }
                map
            }
        }
    };
}

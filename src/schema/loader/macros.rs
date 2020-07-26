#[macro_export]
macro_rules! loader_by_id {
    ($id:ty, $model:ty, $table:ident) => {
        #[async_trait::async_trait]
        impl dataloader::BatchFn<$id, Option<$model>> for crate::schema::Database {
            async fn load(&self, keys: &[$id]) -> std::collections::HashMap<$id, Option<$model>> {
                use diesel::prelude::*;
                let mut map: std::collections::HashMap<$id, Option<$model>> = keys.iter()
                    .cloned()
                    .map(|key| (key, None))
                    .collect();
                let load_result: anyhow::Result<Vec<$model>> = tokio::task::block_in_place(|| {
                    let conn = self.connection()?;
                    Ok(data::$table::table
                        .filter(data::$table::id.eq_any(keys))
                        .load(&conn)?)
                });
                let items = load_result.unwrap_or(vec![]);
                for item in items {
                    map.get_mut(&item.id).unwrap().replace(item);
                }
                map
            }
        }

        #[allow(dead_code)]
        impl $crate::schema::loader::Loader<$id, $model> {
            pub async fn load(&self, key: $id) -> Option<$model> {
                self.loader.load(key).await
            }

            pub async fn load_many(&self, keys: Vec<$id>) -> std::collections::HashMap<$id, Option<$model>> {
                self.loader.load_many(keys).await
            }
        }
    };
}

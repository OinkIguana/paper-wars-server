#[macro_export]
macro_rules! batch_fn {
    (
        $table:ident => $model:ty {
            $($id:ident: $key:ty),+
        }
    ) => {
        #[async_trait::async_trait]
        #[allow(unused_parens)]
        impl dataloader::BatchFn<($($key),+), Option<$model>> for crate::schema::Database {
            async fn load(&self, keys: &[($($key),+)]) -> std::collections::HashMap<($($key),+), Option<$model>> {
                use diesel::prelude::*;
                let mut map: std::collections::HashMap<($($key),+), Option<$model>> =
                    keys.iter().cloned().map(|key| (key, None)).collect();
                let load_result: anyhow::Result<Vec<$model>> = tokio::task::block_in_place(|| {
                    let conn = self.connection()?;
                    Ok(keys
                        .iter()
                        .cloned()
                        .map(|($($id),+): ($($key),+)| -> Box::<dyn diesel::BoxableExpression<data::$table::table, diesel::pg::Pg, SqlType = diesel::sql_types::Bool> + Send> {
                            vec![$(
                                Box::new(data::$table::$id.eq($id))
                                    as Box::<dyn diesel::BoxableExpression<data::$table::table, diesel::pg::Pg, SqlType = diesel::sql_types::Bool> + Send>
                            ),+]
                                .into_iter()
                                .fold_first(|a, b| Box::new(a.and(b)))
                                .unwrap()
                        })
                        .fold(
                            data::$table::table.into_boxed(),
                            |query, filter| query.or_filter(filter)
                        )
                        .load(&conn)?)
                });
                let items = load_result.unwrap_or(vec![]);
                for item in items {
                    map.get_mut(&$crate::schema::loader::traits::BatchFnItem::key(&item)).unwrap().replace(item);
                }
                map
            }
        }

        #[allow(unused_parens)]
        impl $crate::schema::loader::traits::BatchFnItem for $model {
            type Key = ($($key),+);

            fn key(&self) -> Self::Key {
                ($(self.$id.clone()),+)
            }
        }
    };
}

#[macro_export]
macro_rules! join {
    ($table:ident => $name:ident ($key:ident: $id:ty) -> $model:ty) => {
        pub async fn $name(&self, $key: &$id) -> Vec<$model> {
            use diesel::prelude::*;

            let load_result: anyhow::Result<Vec<$model>> = tokio::task::block_in_place(|| {
                let conn = self.database.connection()?;
                Ok(data::$table::table
                    .filter(data::$table::$key.eq($key))
                    .load(&conn)?)
            });

            let items = load_result.unwrap_or(vec![]);
            self.prime_many(items.clone()).await;
            items
        }
    }
}

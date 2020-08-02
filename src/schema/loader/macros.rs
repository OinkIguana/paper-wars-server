#[macro_export]
macro_rules! batch_fn {
    (
        $table:ident => $model:ty {
            $($id:ident: $key:ty),+
        }
    ) => {
        #[allow(unused_parens)]
        impl dataloader::sync::BatchFn<($($key),+), Option<$model>> for crate::schema::Database {
            fn load(&self, keys: &[($($key),+)]) -> std::collections::HashMap<($($key),+), Option<$model>> {
                use diesel::prelude::*;
                let mut map: std::collections::HashMap<($($key),+), Option<$model>> =
                    keys.iter().cloned().map(|key| (key, None)).collect();
                let items : Vec<$model> = self.connection()
                    .and_then(|conn| Ok(keys
                        .iter()
                        .cloned()
                        .map(|($($id),+): ($($key),+)| -> Box::<dyn diesel::BoxableExpression<data::$table::table, diesel::pg::Pg, SqlType = diesel::sql_types::Bool> + Send> {
                            let mut parts = vec![$(
                                Box::new(data::$table::$id.eq($id))
                                    as Box::<dyn diesel::BoxableExpression<data::$table::table, diesel::pg::Pg, SqlType = diesel::sql_types::Bool> + Send>
                            ),+];

                            let first = parts.pop().unwrap();
                            parts
                                .into_iter()
                                .fold(first, |a, b| Box::new(a.and(b)))
                        })
                        .fold(
                            data::$table::table.into_boxed(),
                            |query, filter| query.or_filter(filter)
                        )
                        .load(&conn)?))
                    .unwrap_or(vec![]);
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
    ($table:ident => $name:ident ($($key:ident: $id:ty),+) -> $model:ty) => {
        pub fn $name(&self, $($key: &$id),+) -> Vec<$model> {
            use diesel::prelude::*;
            let items: Vec<$model> = self.database.connection()
                .and_then(|conn| Ok(data::$table::table
                    $( .filter(data::$table::$key.eq($key)) )+
                    .load(&conn)?))
                .unwrap_or(vec![]);
            self.prime_many(items.clone());
            items
        }
    };
}

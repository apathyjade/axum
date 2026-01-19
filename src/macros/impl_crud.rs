#[macro_export]
macro_rules! impl_crud {
    ($model:ty, $table:path) => {
        impl $model {
            /// 插入新记录
            pub fn create(
                conn: &mut PgConnection,
                item: <$model as diesel::prelude::Insertable<$table>>::Values,
            ) -> diesel::QueryResult<Self> {
                use diesel::prelude::*;
                diesel::insert_into($table).values(item).get_result(conn)
            }

            /// 根据主键查找记录
            ///
            /// 注意：主键类型需与表定义一致（通常为 i32/i64/Uuid）
            pub fn find<PK>(conn: &mut PgConnection, id: PK) -> diesel::QueryResult<Self>
            where
                PK: diesel::prelude::Expression<diesel::sql_types::Integer> + Clone,
                diesel::dsl::Eq<
                    <Self as diesel::Table>::PrimaryKey,
                    diesel::helper_types::AsExprOf<PK, diesel::sql_types::Integer>,
                >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                use diesel::prelude::*;
                $table.find(id).first::<Self>(conn)
            }

            /// 更新记录（需实现 AsChangeset）
            pub fn update(
                conn: &mut PgConnection,
                id: i32, // 可根据需要泛化
                changes: <$model as diesel::prelude::AsChangeset>::Changeset,
            ) -> diesel::QueryResult<Self> {
                use diesel::prelude::*;
                diesel::update($table.find(id))
                    .set(changes)
                    .get_result(conn)
            }

            /// 删除记录
            pub fn delete(conn: &mut PgConnection, id: i32) -> diesel::QueryResult<usize> {
                use diesel::prelude::*;
                diesel::delete($table.find(id)).execute(conn)
            }

            /// 获取所有记录（慎用于大表！）
            pub fn all(conn: &mut PgConnection) -> diesel::QueryResult<Vec<Self>> {
                use diesel::prelude::*;
                $table.load::<Self>(conn)
            }
        }
    };
}

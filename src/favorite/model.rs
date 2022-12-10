use crate::{
    error::AppError,
    schema::{favorites, users},
};
use diesel::{
    EqAll, ExpressionMethods, Identifiable, Insertable, PgConnection, QueryDsl, Queryable,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = favorites)]
pub struct Favorite {
    pub id: i32,
    pub user_id: i32,
    pub album_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = favorites)]
pub struct InsertFavorite {
    pub user_id: i32,
    pub album_id: i32,
}

pub struct DeleteFavorite {
    pub user_id: i32,
    pub album_id: i32,
}

// static methods
impl Favorite {
    pub fn create(conn: &mut PgConnection, record: &InsertFavorite) -> Result<usize, AppError> {
        let id = diesel::insert_into(favorites::table)
            .values(record)
            .execute(conn)?;
        Ok(id)
    }

    pub fn delete(conn: &mut PgConnection, record: &DeleteFavorite) -> Result<usize, AppError> {
        let item = diesel::delete(favorites::table)
            .filter(favorites::user_id.eq_all(record.user_id))
            .filter(favorites::album_id.eq_all(record.album_id))
            .execute(conn)?;
        Ok(item)
    }

    pub fn get_favorites_by_username(
        conn: &mut PgConnection,
        username: &str,
    ) -> Result<Vec<i32>, AppError> {
        let ids: Vec<i32> = favorites::table
            .inner_join(users::table)
            .filter(users::username.eq(username))
            .select(favorites::album_id)
            .load(conn)?;
        Ok(ids)
    }
}

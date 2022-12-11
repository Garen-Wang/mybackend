use crate::{
    album::model::Album,
    error::AppError,
    schema::{albums, favorites, users},
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
    pub fn create(conn: &mut PgConnection, user_id: i32, album_id: i32) -> Result<usize, AppError> {
        let record = InsertFavorite { user_id, album_id };
        let id = diesel::insert_into(favorites::table)
            .values(record)
            .execute(conn)?;
        Ok(id)
    }

    pub fn delete(conn: &mut PgConnection, user_id: i32, album_id: i32) -> Result<usize, AppError> {
        let record = DeleteFavorite { user_id, album_id };
        let item = diesel::delete(favorites::table)
            .filter(favorites::user_id.eq_all(record.user_id))
            .filter(favorites::album_id.eq_all(record.album_id))
            .execute(conn)?;
        Ok(item)
    }

    pub fn find_by_username(conn: &mut PgConnection, username: &str) -> Result<Vec<i32>, AppError> {
        let ids: Vec<i32> = favorites::table
            .inner_join(users::table)
            .filter(users::username.eq(username))
            .select(favorites::album_id)
            .load(conn)?;
        Ok(ids)
    }

    pub fn find_by_user_id(conn: &mut PgConnection, user_id: i32) -> Result<Vec<Album>, AppError> {
        let ids = favorites::table
            .filter(favorites::user_id.eq(user_id))
            .select(favorites::album_id)
            .get_results::<i32>(conn)?;
        let albums = albums::table
            .filter(albums::id.eq_any(ids))
            .load::<Album>(conn)?;
        Ok(albums)
    }
}

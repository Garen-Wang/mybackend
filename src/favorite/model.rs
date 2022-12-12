use crate::{
    album::model::Album,
    artist::model::Artist,
    error::AppError,
    schema::{albums, favorite_albums, favorite_artists, singers},
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = favorite_albums)]
pub struct FavoriteAlbum {
    pub id: i32,
    pub user_id: i32,
    pub album_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = favorite_albums)]
pub struct InsertFavoriteAlbum {
    pub user_id: i32,
    pub album_id: i32,
}

pub struct DeleteFavoriteAlbum {
    pub user_id: i32,
    pub album_id: i32,
}

// static methods
impl FavoriteAlbum {
    pub fn create(conn: &mut PgConnection, user_id: i32, album_id: i32) -> Result<usize, AppError> {
        let record = InsertFavoriteAlbum { user_id, album_id };
        let id = diesel::insert_into(favorite_albums::table)
            .values(record)
            .execute(conn)?;
        Ok(id)
    }

    pub fn delete(conn: &mut PgConnection, user_id: i32, album_id: i32) -> Result<usize, AppError> {
        let record = DeleteFavoriteAlbum { user_id, album_id };
        let item = diesel::delete(favorite_albums::table)
            .filter(favorite_albums::user_id.eq_all(record.user_id))
            .filter(favorite_albums::album_id.eq_all(record.album_id))
            .execute(conn)?;
        Ok(item)
    }

    // pub fn find_by_username(conn: &mut PgConnection, username: &str) -> Result<Vec<i32>, AppError> {
    //     let ids: Vec<i32> = favorite_albums::table
    //         .inner_join(users::table)
    //         .filter(users::username.eq(username))
    //         .select(favorite_albums::album_id)
    //         .load(conn)?;
    //     Ok(ids)
    // }

    pub fn find_by_user_id(conn: &mut PgConnection, user_id: i32) -> Result<Vec<Album>, AppError> {
        let ids = favorite_albums::table
            .filter(favorite_albums::user_id.eq(user_id))
            .select(favorite_albums::album_id)
            .get_results::<i32>(conn)?;
        let albums = albums::table
            .filter(albums::id.eq_any(ids))
            .load::<Album>(conn)?;
        Ok(albums)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = favorite_artists)]
pub struct FavoriteArtist {
    pub id: i32,
    pub user_id: i32,
    pub artist_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = favorite_artists)]
pub struct InsertFavoriteArtist {
    pub user_id: i32,
    pub artist_id: i32,
}

pub struct DeleteFavoriteArtist {
    pub user_id: i32,
    pub artist_id: i32,
}

// static methods
impl FavoriteArtist {
    pub fn create(
        conn: &mut PgConnection,
        user_id: i32,
        artist_id: i32,
    ) -> Result<usize, AppError> {
        let record = InsertFavoriteArtist { user_id, artist_id };
        let id = diesel::insert_into(favorite_artists::table)
            .values(record)
            .execute(conn)?;
        Ok(id)
    }

    pub fn delete(
        conn: &mut PgConnection,
        user_id: i32,
        artist_id: i32,
    ) -> Result<usize, AppError> {
        let record = DeleteFavoriteArtist { user_id, artist_id };
        let item = diesel::delete(favorite_artists::table)
            .filter(favorite_artists::user_id.eq_all(record.user_id))
            .filter(favorite_artists::artist_id.eq_all(record.artist_id))
            .execute(conn)?;
        Ok(item)
    }

    // pub fn find_by_username(conn: &mut PgConnection, username: &str) -> Result<Vec<i32>, AppError> {
    //     let ids: Vec<i32> = favorite_artists::table
    //         .inner_join(users::table)
    //         .filter(users::username.eq(username))
    //         .select(favorite_artists::artist_id)
    //         .load(conn)?;
    //     Ok(ids)
    // }

    pub fn find_by_user_id(conn: &mut PgConnection, user_id: i32) -> Result<Vec<Artist>, AppError> {
        let ids = favorite_artists::table
            .filter(favorite_artists::user_id.eq(user_id))
            .select(favorite_artists::artist_id)
            .get_results::<i32>(conn)?;
        let singers = singers::table
            .filter(singers::id.eq_any(ids))
            .load::<Artist>(conn)?;
        Ok(singers)
    }
}

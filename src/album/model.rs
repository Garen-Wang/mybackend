use crate::{error::AppError, schema::{albums, tracks}};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = albums)]
pub struct Album {
    pub id: i32,
    pub name: String,
    pub artist_id: i32,
    pub agreed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = albums)]
pub struct InsertAlbum<'a> {
    pub name: &'a str,
    pub artist_id: i32,
    pub agreed: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = albums)]
pub struct UpdateAlbum {
    pub name: Option<String>,
    pub agreed: Option<bool>,
}

// static methods
impl Album {
    pub fn find(conn: &mut PgConnection, album_id: i32) -> Result<Album, AppError> {
        let album = albums::table.find(album_id).first(conn)?;
        Ok(album)
    }

    pub fn find_by_aritst(conn: &mut PgConnection, artist_id: i32) -> Result<Vec<Album>, AppError> {
        let albums = albums::table
            .filter(albums::artist_id.eq(artist_id))
            .get_results::<Album>(conn)?;
        Ok(albums)
    }

    pub fn create(conn: &mut PgConnection, artist_id: i32, name: &str, agreed: bool) -> Result<Album, AppError> {
        let album = diesel::insert_into(albums::table)
            .values(InsertAlbum { name, artist_id, agreed })
            .get_result::<Album>(conn)?;
        Ok(album)
    }

    pub fn search(conn: &mut PgConnection, name: String) -> Result<Vec<Album>, AppError> {
        let temp = format!("{}%", name);
        let albums: Vec<Album> = albums::table
            .filter(albums::name.like(temp))
            .get_results(conn)?;
        Ok(albums)
    }

    pub fn delete(conn: &mut PgConnection, album_id: i32) -> Result<usize, AppError> {
        let item = diesel::delete(albums::table)
            .filter(albums::id.eq_all(album_id))
            .execute(conn)?;
        Ok(item)
    }

    pub fn issue(conn: &mut PgConnection, album_id: i32) -> Result<Self, AppError> {
        let changeset = UpdateAlbum {
            agreed: Some(true),
            name: None,
        };
        let album = diesel::update(albums::table.find(album_id))
            .set(changeset)
            .get_result::<Album>(conn)?;
        Ok(album)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = tracks)]
pub struct Track {
    pub id: i32,
    pub name: String,
    pub artist_id: i32,
    pub album_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = tracks)]
pub struct InsertTrack<'a> {
    pub name: &'a str,
    pub artist_id: i32,
    pub album_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = tracks)]
pub struct UpdateTrack {
    pub name: Option<String>,
}

// static methods
impl Track {
    pub fn find(conn: &mut PgConnection, track_id: i32) -> Result<Track, AppError> {
        let track = tracks::table.find(track_id).first(conn)?;
        Ok(track)
    }

    pub fn find_by_album(conn: &mut PgConnection, album_id: i32) -> Result<Vec<Track>, AppError> {
        let tracks = tracks::table
        .filter(tracks::album_id.eq(album_id)).get_results::<Track>(conn)?;
        Ok(tracks)
    }

    pub fn create(
        conn: &mut PgConnection,
        name: &str,
        artist_id: i32,
        album_id: i32,
    ) -> Result<Track, AppError> {
        let track = diesel::insert_into(tracks::table)
            .values(InsertTrack { name, artist_id, album_id })
            .get_result::<Track>(conn)?;
        Ok(track)
    }
}
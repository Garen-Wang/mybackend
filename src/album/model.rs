use chrono::NaiveDateTime;
use diesel::{Queryable, Identifiable, Insertable, AsChangeset, PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};
use serde::{Serialize, Deserialize};
use crate::{schema::albums, error::AppError};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = albums)]
pub struct Album {
    pub id: i32,
    pub name: String,
    pub artist_id: i32,
    pub last_playback: Option<NaiveDateTime>,
    pub agreed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = albums)]
pub struct InsertAlbum<'a> {
    pub name: &'a str,
    pub artist_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = albums)]
pub struct UpdateAlbum {
    pub name: Option<String>,
    pub last_playback: Option<NaiveDateTime>,
    pub agreed: Option<bool>,
}

// static methods
impl Album {
    pub fn find(conn: &mut PgConnection, album_id: i32) -> Result<Album, AppError> {
        let album = albums::table.find(album_id).first(conn)?;
        Ok(album)
    }

    pub fn search(conn: &mut PgConnection, name: String) -> Result<Vec<Album>, AppError> {
        let temp = format!("{}%", name);
        let albums: Vec<Album> = albums::table.filter(albums::name.like(temp)).get_results(conn)?;
        Ok(albums)
    }
}
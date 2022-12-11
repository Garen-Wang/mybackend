use crate::{error::AppError, schema::albums};
use chrono::NaiveDateTime;
use diesel::{
    AsChangeset, EqAll, Identifiable, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl,
    TextExpressionMethods,
};
use serde::{Deserialize, Serialize};

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
            last_playback: None,
        };
        let album = diesel::update(
            albums::table.find(album_id)
        )
        .set(changeset)
        .get_result::<Album>(conn)?;
        Ok(album)
    }
}

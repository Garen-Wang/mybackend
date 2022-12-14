use crate::{error::AppError, schema::artists};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = artists)]
pub struct Artist {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = artists)]
pub struct InsertArtist<'a> {
    pub name: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = artists)]
pub struct UpdateArtist {
    pub name: Option<String>,
}

// static methods
impl Artist {
    pub fn find(conn: &mut PgConnection, artist_id: i32) -> Result<Artist, AppError> {
        let artist = artists::table.find(artist_id).first(conn)?;
        Ok(artist)
    }

    pub fn delete(conn: &mut PgConnection, artist_id: i32) -> Result<usize, AppError> {
        let item = diesel::delete(artists::table)
            .filter(artists::id.eq_all(artist_id))
            .execute(conn)?;
        Ok(item)
    }

    pub fn find_by_name(conn: &mut PgConnection, artist_name: &str) -> Result<Artist, AppError> {
        let artist = artists::table.filter(artists::name.eq(artist_name)).first(conn)?;
        Ok(artist)
    }

    pub fn search(conn: &mut PgConnection, name: String) -> Result<Vec<Artist>, AppError> {
        let temp = format!("{}%", name);
        let artists: Vec<Artist> = artists::table
            .filter(artists::name.like(temp))
            .get_results(conn)?;
        Ok(artists)
    }

    pub fn create(conn: &mut PgConnection, name: &str) -> Result<Artist, AppError> {
        let artist = diesel::insert_into(artists::table).values(InsertArtist { name }).get_result::<Artist>(conn)?;
        Ok(artist)
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Artist>, AppError> {
        let artists = artists::table.get_results::<Artist>(conn)?;
        Ok(artists)
    }
}

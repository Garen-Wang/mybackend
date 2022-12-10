use diesel::{Queryable, Identifiable, Insertable, AsChangeset, PgConnection, QueryDsl, RunQueryDsl, TextExpressionMethods};
use serde::{Serialize, Deserialize};
use crate::{schema::artists, error::AppError};

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

    pub fn search(conn: &mut PgConnection, name: String) -> Result<Vec<Artist>, AppError> {
        let temp = format!("{}%", name);
        let artists: Vec<Artist> = artists::table.filter(artists::name.like(temp)).get_results(conn)?;
        Ok(artists)
    }
}

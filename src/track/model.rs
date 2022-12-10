use chrono::NaiveDateTime;
use diesel::{Queryable, Identifiable, Insertable, AsChangeset, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Serialize, Deserialize};
use crate::{schema::tracks, error::AppError};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = tracks)]
pub struct Track {
    pub id: i32,
    pub name: String,
    pub time_length: i32,
    pub last_time: Option<i32>,
    pub last_playback: Option<NaiveDateTime>,
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
    pub last_time: Option<i32>,
    pub last_playback: Option<NaiveDateTime>,
}

// static methods
impl Track {
    pub fn find(conn: &mut PgConnection, track_id: i32) -> Result<Track, AppError> {
        let track = tracks::table.find(track_id).first(conn)?;
        Ok(track)
    }
}

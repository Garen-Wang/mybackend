use crate::{error::AppError, schema::{user_track_history, user_album_history}};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = user_track_history)]
pub struct UserTrackHistory {
    pub id: i32,
    pub user_id: i32,
    pub track_id: i32,
    pub last_time: Option<i32>,
    pub last_date: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = user_track_history)]
pub struct InsertUserTrackHistory {
    pub user_id: i32,
    pub track_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = user_track_history)]
pub struct UpdateUserTrackHistory {
    pub last_time: Option<i32>,
    pub last_date: Option<NaiveDateTime>,
}

pub struct DeleteUserTrackHistory {
    pub user_id: i32,
    pub track_id: i32,
}

impl UserTrackHistory {
    pub fn create(
        conn: &mut PgConnection,
        user_id: i32,
        track_id: i32,
    ) -> Result<usize, AppError> {
        let record = InsertUserTrackHistory {
            user_id,
            track_id,
        };
        let id = diesel::insert_into(user_track_history::table)
            .values(record)
            .execute(conn)?;
        Ok(id)
    }

    pub fn exists(
        conn: &mut PgConnection,
        user_id: i32,
        track_id: i32,
    ) -> Result<bool, AppError> {
        let a = user_track_history::table
        .filter(user_track_history::user_id.eq(user_id))
        .filter(user_track_history::track_id.eq(track_id))
        .get_results::<UserTrackHistory>(conn)?;
        Ok(a.len() > 0)
    }

    pub fn update(
        conn: &mut PgConnection,
        user_id: i32,
        track_id: i32,
        changeset: UpdateUserTrackHistory,
    ) -> Result<UserTrackHistory, AppError> {
        let target = user_track_history::table
            .filter(user_track_history::user_id.eq(user_id))
            .filter(user_track_history::track_id.eq(track_id));
        let x = diesel::update(target).set(changeset).get_result(conn)?;
        Ok(x)
    }

    pub fn delete(conn: &mut PgConnection, user_id: i32, track_id: i32) -> Result<usize, AppError> {
        let record = DeleteUserTrackHistory { user_id, track_id };
        let item = diesel::delete(user_track_history::table)
            .filter(user_track_history::user_id.eq_all(record.user_id))
            .filter(user_track_history::track_id.eq_all(record.track_id))
            .execute(conn)?;
        Ok(item)
    }

    pub fn find_by_user_id(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<Vec<UserTrackHistory>, AppError> {
        let history = user_track_history::table
            .filter(user_track_history::user_id.eq(user_id))
            .get_results::<UserTrackHistory>(conn)?;
        Ok(history)
    }

    pub fn find_by_user_track_id(
        conn: &mut PgConnection,
        user_id: i32,
        track_id: i32,
    ) -> Result<UserTrackHistory, AppError> {
        let history = user_track_history::table
            .filter(user_track_history::user_id.eq(user_id))
            .filter(user_track_history::track_id.eq(track_id))
            .get_result::<UserTrackHistory>(conn)?;
        Ok(history)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = user_album_history)]
pub struct UserAlbumHistory {
    pub id: i32,
    pub user_id: i32,
    pub album_id: i32,
    pub last_time: Option<i32>,
    pub last_date: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = user_album_history)]
pub struct InsertUserAlbumHistory {
    pub user_id: i32,
    pub album_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = user_album_history)]
pub struct UpdateUserAlbumHistory {
    pub last_time: Option<i32>,
    pub last_date: Option<NaiveDateTime>,
}

pub struct DeleteUserAlbumHistory {
    pub user_id: i32,
    pub album_id: i32,
}

impl UserAlbumHistory {
    pub fn create(
        conn: &mut PgConnection,
        user_id: i32,
        album_id: i32,
    ) -> Result<usize, AppError> {
        let record = InsertUserAlbumHistory {
            user_id,
            album_id,
        };
        let id = diesel::insert_into(user_album_history::table)
            .values(record)
            .execute(conn)?;
        Ok(id)
    }

    pub fn exists(
        conn: &mut PgConnection,
        user_id: i32,
        album_id: i32,
    ) -> Result<bool, AppError> {
        let a = user_album_history::table
        .filter(user_album_history::user_id.eq(user_id))
        .filter(user_album_history::album_id.eq(album_id))
        .get_results::<UserAlbumHistory>(conn)?;
        Ok(a.len() > 0)
    }

    pub fn update(
        conn: &mut PgConnection,
        user_id: i32,
        album_id: i32,
        changeset: UpdateUserAlbumHistory,
    ) -> Result<UserAlbumHistory, AppError> {
        let target = user_album_history::table
            .filter(user_album_history::user_id.eq(user_id))
            .filter(user_album_history::album_id.eq(album_id));
        let user = diesel::update(target).set(changeset).get_result(conn)?;
        Ok(user)
    }

    pub fn delete(conn: &mut PgConnection, user_id: i32, album_id: i32) -> Result<usize, AppError> {
        let record = DeleteUserAlbumHistory { user_id, album_id };
        let item = diesel::delete(user_album_history::table)
            .filter(user_album_history::user_id.eq_all(record.user_id))
            .filter(user_album_history::album_id.eq_all(record.album_id))
            .execute(conn)?;
        Ok(item)
    }

    pub fn find_by_user_id(
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<Vec<UserAlbumHistory>, AppError> {
        let history = user_album_history::table
            .filter(user_album_history::user_id.eq(user_id))
            .get_results::<UserAlbumHistory>(conn)?;
        Ok(history)
    }

    pub fn find_by_user_album_id(
        conn: &mut PgConnection,
        user_id: i32,
        album_id: i32,
    ) -> Result<UserAlbumHistory, AppError> {
        let history = user_album_history::table
            .filter(user_album_history::user_id.eq(user_id))
            .filter(user_album_history::album_id.eq(album_id))
            .get_result::<UserAlbumHistory>(conn)?;
        Ok(history)
    }
}

use crate::{error::AppError, schema::comments};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: i32,
    pub album_id: i32,
    pub author_id: i32,
    pub body: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct InsertComment<'a> {
    pub album_id: i32,
    pub author_id: i32,
    pub body: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = comments)]
pub struct UpdateComment {
    pub body: Option<String>,
}

impl Comment {
    pub fn create(
        conn: &mut PgConnection,
        album_id: i32,
        author_id: i32,
        body: &str,
    ) -> Result<usize, AppError> {
        let record = InsertComment {
            album_id,
            author_id,
            body,
        };
        let id = diesel::insert_into(comments::table)
            .values(record)
            .execute(conn)?;
        Ok(id)
    }

    pub fn update(
        conn: &mut PgConnection,
        album_id: i32,
        author_id: i32,
        changeset: UpdateComment,
    ) -> Result<Comment, AppError> {
        let target = comments::table
        .filter(comments::album_id.eq(album_id))
        .filter(comments::author_id.eq(author_id));
        let x = diesel::update(target).set(changeset).get_result(conn)?;
        Ok(x)
    }

    pub fn delete(
        conn: &mut PgConnection,
        album_id: i32,
        author_id: i32,
        comment_id: i32,
    ) -> Result<usize, AppError> {
        let item = diesel::delete(comments::table)
            .filter(comments::album_id.eq(album_id))
            .filter(comments::author_id.eq(author_id))
            .filter(comments::id.eq(comment_id))
            .execute(conn)?;
        Ok(item)
    }

    pub fn find_by_album_id(
        conn: &mut PgConnection,
        album_id: i32,
    ) -> Result<Vec<Comment>, AppError> {
        let comments = comments::table
            .filter(comments::album_id.eq(album_id))
            .get_results::<Comment>(conn)?;
        Ok(comments)
    }
}

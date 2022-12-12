use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use super::model::Comment;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommentResponse {
    pub comments: Vec<CommentContent>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommentContent {
    pub author_id: i32,
    pub body: String,
    pub created_at: NaiveDateTime,
}

impl From<Vec<Comment>> for CommentResponse {
    fn from(comments: Vec<Comment>) -> Self {
        Self {
            comments: comments.into_iter().map(|x| CommentContent {
                author_id: x.author_id,
                body: x.body,
                created_at: x.created_at,
            }).collect()
        }
    }
}
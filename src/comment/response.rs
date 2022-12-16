use serde::{Deserialize, Serialize};

use super::model::Comment;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommentResponse {
    pub comments: Vec<Comment>,
}

impl From<Vec<Comment>> for CommentResponse {
    fn from(comments: Vec<Comment>) -> Self {
        Self { comments }
    }
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentRequest {
    pub comment: CommentRequestInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentRequestInner {
    pub body: String,
}

use serde::{Serialize, Deserialize};

use crate::album::model::Album;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteResponse {
    pub albums: Vec<Album>,
}

impl From<Vec<Album>> for FavoriteResponse {
    fn from(albums: Vec<Album>) -> Self {
        Self { albums }
    }
}
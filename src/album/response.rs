use serde::{Serialize, Deserialize};

use crate::album::model::Album;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumResponse {
    pub albums: Vec<Album>,
}

impl From<Vec<Album>> for AlbumResponse {
    fn from(albums: Vec<Album>) -> Self {
        Self { albums }
    }
}
impl From<Album> for AlbumResponse {
    fn from(album: Album) -> Self {
        Self { albums: vec![album] }
    }
}
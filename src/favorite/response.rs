use serde::{Deserialize, Serialize};

use crate::{album::model::Album, artist::model::Artist};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteAlbumResponse {
    pub albums: Vec<Album>,
}

impl From<Vec<Album>> for FavoriteAlbumResponse {
    fn from(albums: Vec<Album>) -> Self {
        Self { albums }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteArtistResponse {
    pub artists: Vec<Artist>,
}

impl From<Vec<Artist>> for FavoriteArtistResponse {
    fn from(artists: Vec<Artist>) -> Self {
        Self { artists }
    }
}
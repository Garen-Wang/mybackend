use serde::{Deserialize, Serialize};

use crate::album::model::Album;

use super::model::Track;

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
        Self {
            albums: vec![album],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackResponse {
    pub tracks: Vec<Track>,
}

impl From<Vec<Track>> for TrackResponse {
    fn from(tracks: Vec<Track>) -> Self {
        Self { tracks }
    }
}
impl From<Track> for TrackResponse {
    fn from(track: Track) -> Self {
        Self {
            tracks: vec![track],
        }
    }
}

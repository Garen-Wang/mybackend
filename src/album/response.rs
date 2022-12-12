use serde::{Deserialize, Serialize};

use crate::album::model::Album;

use super::model::Track;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumResponse {
    pub albums: Vec<AlbumInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumInfo {
    pub name: String,
    pub artist_id: i32,
}

impl From<Vec<Album>> for AlbumResponse {
    fn from(albums: Vec<Album>) -> Self {
        Self {
            albums: albums
                .into_iter()
                .map(|x| AlbumInfo {
                    name: x.name,
                    artist_id: x.artist_id,
                })
                .collect(),
        }
    }
}
impl From<Album> for AlbumResponse {
    fn from(album: Album) -> Self {
        Self {
            albums: vec![AlbumInfo {
                name: album.name,
                artist_id: album.artist_id,
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackResponse {
    pub tracks: Vec<TrackInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
    pub name: String,
    pub artist_id: i32,
    pub album_id: i32,
}

impl From<Vec<Track>> for TrackResponse {
    fn from(tracks: Vec<Track>) -> Self {
        Self {
            tracks: tracks
                .into_iter()
                .map(|x| TrackInfo {
                    name: x.name,
                    artist_id: x.artist_id,
                    album_id: x.album_id,
                })
                .collect(),
        }
    }
}
impl From<Track> for TrackResponse {
    fn from(track: Track) -> Self {
        Self {
            tracks: vec![TrackInfo {
                name: track.name,
                artist_id: track.artist_id,
                album_id: track.album_id,
            }],
        }
    }
}

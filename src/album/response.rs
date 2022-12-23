use serde::{Deserialize, Serialize};

use crate::{album::model::Album, artist::model::Artist, comment::model::Comment, AppConn};

use super::model::Track;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumResponse {
    pub albums: Vec<AlbumWithTracks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumWithTracks {
    pub album: Album,
    pub artist_name: String,
    pub tracks: Vec<Track>,
    pub comments: Vec<Comment>,
}

impl From<(Album, &mut AppConn)> for AlbumResponse {
    fn from(value: (Album, &mut AppConn)) -> Self {
        let (album, conn) = value;
        Self {
            albums: vec![AlbumWithTracks {
                artist_name: Artist::find(conn, album.artist_id).unwrap().name,
                tracks: Track::find_by_album(conn, album.id).unwrap(),
                comments: Comment::find_by_album_id(conn, album.id).unwrap(),
                album,
            }],
        }
    }
}

impl From<(Vec<Album>, &mut AppConn)> for AlbumResponse {
    fn from(value: (Vec<Album>, &mut AppConn)) -> Self {
        let (albums, conn) = value;
        Self {
            albums: albums
                .into_iter()
                .map(|album| AlbumWithTracks {
                    artist_name: Artist::find(conn, album.artist_id).unwrap().name,
                    tracks: Track::find_by_album(conn, album.id).unwrap(),
                    comments: Comment::find_by_album_id(conn, album.id).unwrap(),
                    album,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackResponse {
    pub tracks: Vec<TrackWithComments>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackWithComments {
    pub track: Track,
    pub album_name: String,
    pub artist_name: String,
    pub comments: Vec<Comment>,
}

impl From<(Vec<Track>, &mut AppConn)> for TrackResponse {
    fn from(value: (Vec<Track>, &mut AppConn)) -> Self {
        let (tracks, conn) = value;
        Self {
            tracks: tracks
                .into_iter()
                .map(|track| TrackWithComments {
                    album_name: Album::find(conn, track.album_id).unwrap().name,
                    artist_name: Artist::find(conn, track.artist_id).unwrap().name,
                    comments: Comment::find_by_album_id(conn, track.album_id).unwrap(),
                    track,
                })
                .collect(),
        }
    }
}
impl From<(Track, &mut AppConn)> for TrackResponse {
    fn from(value: (Track, &mut AppConn)) -> Self {
        let (track, conn) = value;
        Self {
            tracks: vec![TrackWithComments {
                album_name: Album::find(conn, track.album_id).unwrap().name,
                artist_name: Artist::find(conn, track.artist_id).unwrap().name,
                comments: Comment::find_by_album_id(conn, track.album_id).unwrap(),
                track,
            }],
        }
    }
}

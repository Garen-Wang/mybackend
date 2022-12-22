use serde::{Deserialize, Serialize};

use crate::{album::model::Album, artist::model::Artist, AppConn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteAlbumResponse {
    pub albums: Vec<AlbumWithArtistName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumWithArtistName {
    pub album: Album,
    pub artist_name: String,
}

impl From<(Vec<Album>, &mut AppConn)> for FavoriteAlbumResponse {
    fn from(value: (Vec<Album>, &mut AppConn)) -> Self {
        let (albums, conn) = value;
        Self {
            albums: albums
                .into_iter()
                .map(|album| AlbumWithArtistName {
                    artist_name: Artist::find(conn, album.artist_id).unwrap().name,
                    album,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteArtistResponse {
    pub artists: Vec<ArtistWithAlbums>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistWithAlbums {
    pub artist: Artist,
    pub albums: Vec<Album>,
}

impl From<(Vec<Artist>, &mut AppConn)> for FavoriteArtistResponse {
    fn from(value: (Vec<Artist>, &mut AppConn)) -> Self {
        let (artists, conn) = value;
        Self {
            artists: artists
                .into_iter()
                .map(|artist| ArtistWithAlbums {
                    albums: Album::find_by_artist(conn, artist.id).unwrap(),
                    artist,
                })
                .collect(),
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::album::model::Album;
use crate::artist::model::Artist;
use crate::AppConn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistResponse {
    pub artists: Vec<ArtistWithAlbums>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistWithAlbums {
    pub artist: Artist,
    pub albums: Vec<Album>,
}

impl From<(Vec<Artist>, &mut AppConn)> for ArtistResponse {
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

impl From<(Artist, &mut AppConn)> for ArtistResponse {
    fn from(value: (Artist, &mut AppConn)) -> Self {
        let (artist, conn) = value;
        Self {
            artists: vec![ArtistWithAlbums {
                albums: Album::find_by_artist(conn, artist.id).unwrap(),
                artist,
            }],
        }
    }
}

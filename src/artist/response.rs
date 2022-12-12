use serde::{Deserialize, Serialize};

use crate::artist::model::Artist;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistResponse {
    pub artists: Vec<Artist>,
}

impl From<Vec<Artist>> for ArtistResponse {
    fn from(artists: Vec<Artist>) -> Self {
        Self { artists }
    }
}

impl From<Artist> for ArtistResponse {
    fn from(artist: Artist) -> Self {
        Self { artists: vec![artist] }
    }
}
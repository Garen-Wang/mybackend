use serde::{Serialize, Deserialize};

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
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlbumRequest {
    pub new_album: CreateAlbumRequestInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlbumRequestInner {
    pub artist_name: String,
    pub album_name: String,
    pub tracks: Vec<AddTrackRequestInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTrackRequest {
    pub tracks: Vec<AddTrackRequestInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTrackRequestInner {
    pub name: String,
    pub url: String,
}

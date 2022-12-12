use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlbumRequest {
    pub new_album: CreateAlbumRequestInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlbumRequestInner {
    pub artist_id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTrackRequest {
    pub new_track: AddTrackRequestInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTrackRequestInner {
    pub artist_id: i32,
    pub name: String,
}


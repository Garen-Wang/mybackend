use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddArtistRequest {
    pub new_artist: AddArtistRequestInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddArtistRequestInner {
    pub name: String,
}
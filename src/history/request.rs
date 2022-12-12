use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct UpdateUserTrackHistoryRequest {
    pub latest_playback: UpdateUserHistroyRequestInner,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct UpdateUserAlbumHistoryRequest {
    pub latest_playback: UpdateUserHistroyRequestInner,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct UpdateUserHistroyRequestInner {
    pub last_time: Option<i32>,
    pub last_date: Option<NaiveDateTime>,
}
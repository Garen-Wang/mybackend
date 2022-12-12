use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::model::{UserTrackHistory, UserAlbumHistory};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTrackHistoryResponse {
    pub history: Vec<TrackLastPlayback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackLastPlayback {
    pub track_id: i32,
    pub last_time: Option<i32>,
    pub last_date: Option<NaiveDateTime>,
}

impl From<UserTrackHistory> for UserTrackHistoryResponse {
    fn from(info: UserTrackHistory) -> Self {
        Self {
            history: vec![TrackLastPlayback {
                track_id: info.track_id,
                last_time: info.last_time,
                last_date: info.last_date,
            }],
        }
    }
}

impl From<Vec<UserTrackHistory>> for UserTrackHistoryResponse {
    fn from(info: Vec<UserTrackHistory>) -> Self {
        Self {
            history: info
                .into_iter()
                .map(|x| TrackLastPlayback {
                    track_id: x.track_id,
                    last_time: x.last_time,
                    last_date: x.last_date,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAlbumHistoryResponse {
    pub history: Vec<AlbumLastPlayback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumLastPlayback {
    pub album_id: i32,
    pub last_time: Option<i32>,
    pub last_date: Option<NaiveDateTime>,
}

impl From<UserAlbumHistory> for UserAlbumHistoryResponse {
    fn from(info: UserAlbumHistory) -> Self {
        Self {
            history: vec![AlbumLastPlayback {
                album_id: info.album_id,
                last_time: info.last_time,
                last_date: info.last_date,
            }],
        }
    }
}

impl From<Vec<UserAlbumHistory>> for UserAlbumHistoryResponse {
    fn from(info: Vec<UserAlbumHistory>) -> Self {
        Self {
            history: info
                .into_iter()
                .map(|x| AlbumLastPlayback {
                    album_id: x.album_id,
                    last_time: x.last_time,
                    last_date: x.last_date,
                })
                .collect(),
        }
    }
}

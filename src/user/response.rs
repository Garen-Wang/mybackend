use serde::{Deserialize, Serialize};

use super::model::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: AuthUser,
}

impl From<(User, String)> for UserResponse {
    fn from((user, token): (User, String)) -> Self {
        Self {
            user: AuthUser {
                email: user.email,
                token,
                username: user.username,
                bio: user.bio,
                image: user.image,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

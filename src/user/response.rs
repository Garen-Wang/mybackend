use serde::{Deserialize, Serialize};

use super::model::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub users: Vec<AuthUser>,
    pub token: Option<String>,
}

impl From<(User, String)> for UserResponse {
    fn from((user, token): (User, String)) -> Self {
        Self {
            users: vec![AuthUser {
                id: user.id,
                username: user.username,
                email: user.email,
            }],
            token: Some(token),
        }
    }
}

impl From<Vec<User>> for UserResponse {
    fn from(users: Vec<User>) -> Self {
        Self {
            users: users.into_iter().map(|x| AuthUser {
                id: x.id,
                username: x.username,
                email: x.email,
            }).collect(),
            token: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i32,
    pub username: String,
    pub email: String,
}

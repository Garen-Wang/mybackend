use jsonwebtoken::{Header, encode, EncodingKey, decode, TokenData, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

const ONE_HOUR: i64 = 60 * 60;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub expired_at: i64, // timestamp
    pub issued_at:  i64, // timestamp
    pub user_id:    Uuid,
}

impl Claims {
    pub fn new(user_id: Uuid, now: i64) -> Self {
        Self {
            expired_at: now + ONE_HOUR,
            issued_at: now,
            user_id,
        }
    }
}

pub fn generate_token(user_id: Uuid, now: i64) -> jsonwebtoken::errors::Result<String> {
    let claims = Claims::new(user_id, now);
    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
}

pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    decode(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())
}
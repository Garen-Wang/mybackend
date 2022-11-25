use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

const ONE_HOUR: i64 = 60 * 60;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64, // expired at (required)
    pub iat: i64, // issued at (optional)
    pub user_id: i32,
}

impl Claims {
    pub fn new(user_id: i32, now: i64) -> Self {
        Self {
            iat: now,
            exp: now + ONE_HOUR,
            user_id,
        }
    }
}

pub fn generate_token(user_id: i32, now: i64) -> jsonwebtoken::errors::Result<String> {
    let key = b"key";
    let claims = Claims::new(user_id, now);
    let header = Header {
        alg: jsonwebtoken::Algorithm::HS512,
        kid: Some("signing_key".to_owned()),
        ..Default::default()
    };
    encode(&header, &claims, &EncodingKey::from_secret(key))
}

pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    let key = b"key";
    decode(
        token,
        &DecodingKey::from_secret(key),
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    )
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestClaims {
        sub: String,
        company: String,
        exp: usize,
    }

    #[test]
    fn test_encode_decode() {
        let test_claims = TestClaims {
            sub: "b@b.com".to_owned(),
            company: "ACME".to_owned(),
            exp: 10000000000,
        };

        let key = b"secret";

        let header = Header {
            alg: jsonwebtoken::Algorithm::HS512,
            kid: Some("signing_key".to_owned()),
            ..Default::default()
        };

        let token = encode(&header, &test_claims, &EncodingKey::from_secret(key)).unwrap();
        // println!("{:?}", token);

        let token_data = decode::<TestClaims>(
            &token,
            &DecodingKey::from_secret(key),
            &Validation::new(jsonwebtoken::Algorithm::HS512),
        )
        .unwrap();

        // println!("{:?}", token_data.claims);
        // println!("{:?}", token_data.header);
        assert_eq!(token_data.claims, test_claims);
    }
}

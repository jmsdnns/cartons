use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const SECRET_KEY: &str = "our secret key";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn create_token(
    hmac_key: &str,
    username: &str,
    expire: Duration,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();

    let claims = Claims {
        email: username.to_string(),
        exp: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(hmac_key.as_bytes()),
    )?;

    Ok(token)
}

pub fn extract_token(hmac_key: &str, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // We'll use custom validation instead of the usual Validation::default()
    let mut validation = Validation::default();
    validation.leeway = 0; // defaults to 60 seconds. we use 0 to make the point quickly

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(hmac_key.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}

fn main() {
    // Valid token

    println!("VALID TOKEN\n-----------");
    let expire = Duration::hours(24);
    let token = create_token(SECRET_KEY, "myemail@weeee.com", expire).unwrap();
    println!("TOKEN: {:?}", token);

    let claims = extract_token(SECRET_KEY, token.as_str());
    println!("CLAIMS: {:?}\n", claims);

    // Expired token

    println!("EXPIRED TOKEN\n-------------");
    let expire = Duration::seconds(1);
    let token = create_token(SECRET_KEY, "myemail@weeee.com", expire).unwrap();
    println!("TOKEN: {:?}", token);

    // Sleep long enough to expire token
    use std::{thread, time};
    let sleep = time::Duration::from_secs(2);
    println!("SLEEP 2 SECONDS");
    thread::sleep(sleep);

    let claims = extract_token(SECRET_KEY, token.as_str());
    println!("CLAIMS: {:?}\n", claims);
}

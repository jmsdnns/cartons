use anyhow::{anyhow, Context, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub async fn hash_password(password: String) -> Result<String> {
    tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(PasswordHash::generate(Argon2::default(), &password, &salt)
            .map_err(|e| {
                anyhow::anyhow!(e).context(format!("failed to generate password hash: {}", e))
            })
            .unwrap()
            .to_string())
    })
    .await
    .context("panic in generating password hash")?
}

pub async fn verify_password(password: String, password_hash: String) -> Result<()> {
    tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => {
                    anyhow::anyhow!(e).context(format!("unauthorized: {:?}", e))
                }
                _ => anyhow::anyhow!(e).context(format!("failed to verify password hash: {}", e)),
            })
    })
    .await
    .context("panic in verifying password hash")?
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "username: {}\nexp: {}\niat: {}",
            self.username, self.exp, self.iat
        )
    }
}

pub fn create_token(hmac_key: &str, username: &str) -> Result<String> {
    let now = Utc::now();
    let expire = Duration::hours(24);
    //let expire = Duration::seconds(5);
    let claims = Claims {
        username: username.to_string(),
        exp: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let Ok(token) = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(hmac_key.as_bytes()),
    ) else {
        return Err(anyhow!("unauthorized"));
    };

    Ok(token)
}

pub fn extract_token(hmac_key: &str, token: &str) -> Result<Claims> {
    let Ok(token_data) = decode::<Claims>(
        token,
        &DecodingKey::from_secret(hmac_key.as_bytes()),
        &Validation::default(),
    ) else {
        return Err(anyhow!("unauthorized"));
    };

    if token_data.claims.exp < Utc::now().timestamp() as usize {
        return Err(anyhow!("token expired"));
    }

    Ok(token_data.claims)
}

use anyhow::{Context, Result};
use argon2::{password_hash::SaltString, Argon2, PasswordHash};

async fn hash_password(password: String) -> Result<String> {
    tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(PasswordHash::generate(Argon2::default(), &password, &salt)
            .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))
            .unwrap()
            .to_string())
    })
    .await
    .context("panic in generating password hash")?
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => {
                    anyhow::anyhow!("unauthorized")
                }
                _ => anyhow::anyhow!("failed to verify password hash: {}", e),
            })
    })
    .await
    .context("panic in verifying password hash")?
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let password = "meowmeowbeans".to_string();
    println!("Password:  {}", password);

    let hash = hash_password(password.clone()).await?;
    println!("Pass Hash: {:?}", hash);

    let mut guess = "meowmeowbeans".to_string();
    println!("Trying: {}", guess);
    match verify_password(guess, hash.clone()).await {
        Ok(_) => println!("- verified"),
        Err(_) => println!("- nope"),
    }

    guess = "woofwoof".to_string();
    println!("Trying: {}", guess);
    match verify_password(guess, hash.clone()).await {
        Ok(_) => println!("- verified"),
        Err(_) => println!("- nope"),
    }

    Ok(())
}

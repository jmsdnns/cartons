#![allow(dead_code, unused_imports)]

use async_ssh2_tokio::{client::AuthMethod, Error};
use serde::Deserialize;
use std::env;

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

mod pools;

#[derive(Debug, Deserialize)]
struct CliConfig {
    hosts: Vec<String>,
    key_file: String,
    username: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: CliConfig = Figment::new()
        .merge(Toml::file("sshpools.toml"))
        .extract()
        .unwrap();

    println!("CONFIG:");
    println!("- hosts: {:?}", config.hosts);
    println!("- key file: {}", config.key_file);
    println!("- username: {}", config.username);

    // AuthMethod::with_key_file("key_file_name", Some("passphrase"));
    // or
    // AuthMethod::with_key_file("key_file_name", None);
    // or
    // AuthMethod::with_key(key: &str, passphrase: Option<&str>)
    // or
    // AuthMethod::with_password("killakilla");

    let auth = AuthMethod::with_key_file(config.key_file, None);

    let hosts: Vec<&str> = config.hosts.iter().map(|v| v.as_str()).collect();
    let pool = pools::SSHPool::new(hosts, &config.username, &auth).await;

    let cmd = "sleep $(shuf -i 1-3 -n 1) && hostname";
    println!("RESULTS:");
    pools::print_results(pool.exec(cmd).await);

    Ok(())
}

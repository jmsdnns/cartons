use std::io::stdin;

use vibing::{vibing_client::VibingClient, VibeRequest};

pub mod vibing {
    tonic::include_proto!("vibing");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = VibingClient::connect("http://[::1]:5454").await?;

    loop {
        println!("What's going on?");
        let mut comment = String::new();
        stdin().read_line(&mut comment).unwrap();

        let mut vibe = String::new();
        println!("Vibe? (b)ussin or (s)krrt");
        stdin().read_line(&mut vibe).unwrap();

        let comment = comment.trim();
        let v = match vibe.trim().to_lowercase().chars().next().unwrap() {
            'b' => 0,
            's' => 1,
            _ => break,
        };

        let request = tonic::Request::new(VibeRequest {
            comment: String::from(comment),
            vibe: v,
        });
        let response = client.vibe(request).await?;

        println!("[ack] {}\n", response.into_inner().confirmation);
    }

    Ok(())
}

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Musician {
    username: String,
    name: String,
    top_artists: Vec<String>,
    guitars: u8,
    drumsets: u8,
}

fn from_json() -> Result<()> {
    let json_data = r#"
        {
            "username": "jmsdnns",
            "name": "Jms Dnns",
            "top_artists": [
                "Amon Tobin",
                "Fiona Apple",
                "Mars Volta",
                "Dave Brubeck",
                "Anfisa Letyago"
            ],
            "guitars": 7,
            "drumsets": 1
        }"#;

    let m: Musician = serde_json::from_str(json_data)?;

    let top_artists = m
        .top_artists
        .into_iter()
        .collect::<Vec<String>>()
        .join(", ");

    println!("{} loves listening to {}", m.username, top_artists);
    println!("{} also has {} guitars", m.username, m.guitars);

    Ok(())
}

fn to_json() -> Result<()> {
    let m = Musician {
        username: "jmsdnns".to_string(),
        name: "Jms Dnns".to_string(),
        top_artists: vec![
            "Amon Tobin".to_string(),
            "Fiona Apple".to_string(),
            "Mars Volta".to_string(),
            "Dave Brubeck".to_string(),
            "Anfisa Letyago".to_string(),
        ],
        guitars: 5,
        drumsets: 2,
    };

    let json_str = serde_json::to_string(&m)?;
    println!("JSON:\n{}", json_str);

    Ok(())
}

fn main() -> Result<()> {
    from_json()?;
    to_json()?;

    Ok(())
}

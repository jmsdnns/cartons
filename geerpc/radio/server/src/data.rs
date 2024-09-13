use serde::Deserialize;
use std::fs::File;

#[derive(Clone, Debug, Deserialize)]
pub struct BandMember {
    pub name: String,
    pub role: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BandInfo {
    pub name: String,
    pub bio: String,
    pub members: Vec<BandMember>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Song {
    pub name: String,
    pub band: String,
    pub album: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BandData {
    pub bandinfos: Vec<BandInfo>,
    pub songs: Vec<Song>,
}

#[allow(dead_code)]
pub fn load() -> BandData {
    let file = File::open("radiodb.json").expect("failed to open data file");

    let decoded: BandData = serde_json::from_reader(&file).expect("failed to deserialize features");

    decoded
}

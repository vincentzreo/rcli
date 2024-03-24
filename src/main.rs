use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "Kit Number")]
    pub number: u8,
}
impl Player {
    pub fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }
}

fn main() -> Result<()> {
    let file = File::open("assets/juventus.csv")?;
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
        let player: Player = result?;
        println!("{}", player.to_json()?);
    }
    Ok(())
}

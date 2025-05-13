use std::fs::File;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Player{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Positions")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    number: u8,
}

fn main() -> Result<()>{
    let file = File::open("assets/juventus.csv")?;
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize(){
        let player: Player = result?;
        println!("{}", player.name);
    }
    Ok(())
}


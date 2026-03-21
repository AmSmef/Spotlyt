use serde::{Deserialize, Serialize};
use unidecode::unidecode;

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Concert {
    pub artist_name: String,
    pub event_name: String,
    pub venue: String,
    pub city: String,
    pub date: String,
}

// normalises every artist name to be lowercase, without accents and without punctuation
// this is used when comparing artist names between spotify and ticketmaster, as there's a chance they may be stored differently
// like "AC/DC" vs "ACDC", or "Fred Again.." vs "Fred Again"
pub fn normalise_artist_name(name: &str) -> String {
    unidecode(name)
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}
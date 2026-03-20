use serde::{Deserialize, Serialize};

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
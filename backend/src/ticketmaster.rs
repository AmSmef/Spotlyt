use reqwest::Client;
use crate::types::{Artist, Concert, normalise_artist_name};
use std::collections::HashSet;

pub async fn get_concerts(artists: &[Artist], country_code: &str) -> Result<Vec<Concert>, String> {

    // read tm api key from .env
    let api_key = std::env::var("TM_ID")
    .map_err(|_| "Missing TicketMaster API Key".to_string())?;

    let client = Client::new();
    let mut concerts: Vec<Concert> = Vec::new();

    // loop over artists, making http requests to ticketmaster api
    for artist in artists {
        let response = client
        .get("https://app.ticketmaster.com/discovery/v2/events.json")
        .query(&[
            ("apikey", api_key.as_str()),
            ("keyword", artist.name.as_str()),
            ("countryCode", country_code),
            ("classificationName", "music"),
            // 5 results per artist (can update once we have better location setting)
            ("size", "5"),
        ])
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

        // parse response into json
        let json: serde_json::Value = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

        let events = match json["_embedded"]["events"].as_array() {
            Some(e) => e,
            None => continue,
        };

        for event in events {

            let event_name = event["name"]
            .as_str()
            .unwrap_or("Unknown Event")
            .to_string();

            let venue = event["_embedded"]["venues"][0]["name"]
            .as_str()
            .unwrap_or("Unknown Venue")
            .to_string();

            let city = event["_embedded"]["venues"][0]["city"]["name"]
            .as_str()
            .unwrap_or("Unknown City")
            .to_string();

            let date = event["dates"]["start"]["localDate"]
            .as_str()
            .unwrap_or("Unknown Date")
            .to_string();

            // name matching logic between spotify and ticketmaster
            let attractions = event["_embedded"]["attractions"]
            .as_array()
            .map(|a| a.iter()
                .filter_map(|x| x["name"].as_str())
                .map(|n| normalise_artist_name(n))
                .collect::<Vec<String>>()
            )
            .unwrap_or_default();

            // add concert data to vector if the artist is a name attraction for the event
            let normalised_artist = normalise_artist_name(&artist.name);
            if attractions.contains(&normalised_artist) {
                concerts.push(Concert {
                    artist_name: artist.name.clone(),
                    event_name,
                    venue,
                    city,
                    date,
                });
            }
        }
    }

    // occasionally the same concert will be listed twice on ticketmaster
    // like standard tickets vs premium etc etc
    // this deduplicates, so no concert appears in the vector twice
    // I may actually end up removing this, if ticketmaster have duplicates then why shouldn't I???
    let mut seen = HashSet::new();
    let concerts: Vec<Concert> = concerts
        .into_iter()
        .filter(|c| {
            let key = format!("{}-{}-{}", c.artist_name, c.venue, c.date);
            seen.insert(key)
        })
        .collect();
                
    Ok(concerts)
}
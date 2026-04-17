use reqwest::Client;
use crate::types::{Artist, Concert, normalise_artist_name};
use std::collections::HashMap;

pub async fn get_concerts(artists: &[Artist], country_code: &str) -> Result<Vec<Concert>, String> {

    // read tm api key from .env
    let api_key = std::env::var("TM_ID")
    .map_err(|_| "Missing TicketMaster API Key".to_string())?;

    let client = Client::new();

    // keyed by (event_name, venue, date) to merge artists appearing at the same event
    let mut concert_map: HashMap<String, Concert> = HashMap::new();

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

            let image_url = event["images"]
                .as_array()
                .and_then(|images| {
                    let mut matching: Vec<_> = images
                        .iter()
                        .filter(|img| img["ratio"].as_str() == Some("16_9"))
                        .collect();
                    matching.sort_by(|a, b| {
                        let w_a = a["width"].as_u64().unwrap_or(0);
                        let w_b = b["width"].as_u64().unwrap_or(0);
                        w_b.cmp(&w_a)
                    });
                    matching
                        .first()
                        .or(images.first().as_ref())
                        .and_then(|img| img["url"].as_str())
                        .map(|s| s.to_string())
                });

            // name matching logic between spotify and ticketmaster
            let attractions = event["_embedded"]["attractions"]
            .as_array()
            .map(|a| a.iter()
                .filter_map(|x| x["name"].as_str())
                .map(|n| normalise_artist_name(n))
                .collect::<Vec<String>>()
            )
            .unwrap_or_default();

            // add concert data if the artist is a named attraction for the event
            let normalised_artist = normalise_artist_name(&artist.name);
            if attractions.contains(&normalised_artist) {
                let key = format!("{}-{}-{}", event_name, venue, date);

                concert_map.entry(key)
                    .and_modify(|existing| {
                        if !existing.artist_names.contains(&artist.name) {
                            existing.artist_names.push(artist.name.clone());
                        }
                    })
                    .or_insert(Concert {
                        artist_names: vec![artist.name.clone()],
                        event_name,
                        venue,
                        city,
                        date,
                        image_url,
                    });
            }
        }
    }

    let concerts: Vec<Concert> = concert_map.into_values().collect();

    Ok(concerts)
}
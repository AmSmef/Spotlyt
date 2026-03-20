mod auth;
mod types;
mod spotify;
mod ticketmaster;

#[tauri::command]
async fn login() -> Result<String, String> {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let env_path = manifest_dir.join("../.env");
    dotenvy::from_path(env_path).ok();

    let token = auth::authenticate().await?;
    let artists = spotify::get_top_artists(&token).await?;
    let concerts = ticketmaster::get_concerts(&artists, "GB").await?;

    if concerts.is_empty() {
        println!("No upcoming concerts found in your area");
    } else {
        println!("\nRecommended Concerts\n");
        for concert in &concerts {
            println!("Artist: {}", concert.artist_name);
            println!("Event:  {}", concert.event_name);
            println!("Venue:  {}", concert.venue);
            println!("City:   {}", concert.city);
            println!("Date:   {}", concert.date);
            println!("---");
        }
    }

    Ok("Done".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
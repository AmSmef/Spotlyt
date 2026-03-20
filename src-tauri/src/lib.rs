mod auth;
mod types;
mod spotify;

#[tauri::command]
async fn login() -> Result<String, String> {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let env_path = manifest_dir.join("../.env");
    dotenvy::from_path(env_path).ok();

    let token = auth::authenticate().await?;
    let artists = spotify::get_top_artists(&token).await?;

    for artist in artists {
        println!("Artist: {}", artist.name);
    }

    Ok(token)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
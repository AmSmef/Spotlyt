# Spotlyt

Spotlyt is a tool for finding concerts from your favourite artists.

## Yesterday
- `auth.rs` — Spotify OAuth flow, listens on port 8000 for callback
- `spotify.rs` — fetches top 20 artists using user-top-read scope
- `ticketmaster.rs` — searches concerts by artist, deduplicates results
- `types.rs` — Artist, Concert structs + normalise_artist_name()
- `lib.rs` — wires everything together, currently hardcoded to GB

## Today
- Build React frontend to display concerts
- Make country code dynamic (user input)
- Token caching so user doesn't log in every time

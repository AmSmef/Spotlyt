# Spotlyt
Spotlyt is a tool for finding concerts from your favourite artists.

## Yesterday
- Migrated from Tauri to Axum, restructured project into `backend/` and `frontend/` monorepo
- `main.rs` — Axum HTTP server with three routes: `/auth/login`, `/auth/callback`, `/concerts`
- `auth.rs` — refactored into `generate_auth_url()` and `exchange_code()`, with shared `build_oauth_client()` helper
- Added shared `AppState` (CSRF token + access token)
- Added CORS layer

## Today
- Build React frontend (country code input + concert display)
- Wire frontend up to the three Axum endpoints and test e2e

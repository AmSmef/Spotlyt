# Spotlyt
Spotlyt is a tool for finding concerts from your favourite artists.

## Yesterday
- Refactored `App.tsx` into separate page components (`Landing`, `CountrySelection`, `Loading`, `Home`, `Error`) with `Icons.tsx`
- introduced `AppError` enum that returns proper HTTP status codes (`401`, `500`)
- Removed leftover Tauri root-level `package.json` and `tsconfig`
- Added `backend/target/` to `.gitignore` and purged it from the repo

## Today
- Set up PostgreSQL + sqlx with migrations
- Implement Google OAuth for Spotlyt accounts
- Implement email/password signup alongside Google
- User profile schema: users, oauth_accounts, email_accounts, spotify_connections

Next up is updating the frontend to use the new auth flow — register/login first, then link Spotify

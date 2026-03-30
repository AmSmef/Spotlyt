# Frontend Auth Redesign

**Date:** 2026-03-30

## Context

The backend now has a complete multi-provider auth system (username/password, Google OAuth, Spotify OAuth linking) backed by PostgreSQL. The frontend is a minimal proof-of-concept that only supports the old direct-Spotify-login flow (`GET /auth/login` → Spotify OAuth). It needs to be redesigned to integrate with the new auth system.

The app is a linear wizard: authenticate → link Spotify → pick country → view concerts. The existing `useState`-based screen machine is the right pattern to extend.

---

## Screen Flow

```
App loads
  └─ GET /users/me (check session)
      ├─ Logged in → GET /auth/spotify/status
      │                ├─ linked   → "country" screen
      │                └─ unlinked → "spotify-link" screen
      └─ Not logged in → "auth" screen

"auth" screen
  ├─ Login tab    → POST /auth/login    → checkSpotifyStatus() → route
  ├─ Register tab → POST /auth/register → checkSpotifyStatus() → route
  └─ Google btn   → redirect to /auth/google
                     └─ ?loggedin=true  → checkSpotifyStatus() → route

"spotify-link" screen
  └─ "Connect Spotify" btn → redirect to /auth/spotify/link
                              └─ ?spotify_linked=true → "country" screen

"country" → "loading" → "results" / "error"  (unchanged)
```

---

## Architecture

### Screen type

```ts
type Screen = "init" | "auth" | "spotify-link" | "country" | "loading" | "results" | "error"
```

`"init"` is the transient on-load state while the session check resolves (brief spinner). `"landing"` is removed.

### State in App.tsx

No new state shape changes — same `useState` hooks. Add `user` state to hold the current user profile returned from `/users/me` (used to display display name, etc.):

```ts
const [user, setUser] = useState<User | null>(null)
```

### New files

| File | Purpose |
|------|---------|
| `src/api.ts` | Shared fetch helpers — all calls use `credentials: 'include'` |
| `src/pages/Auth.tsx` | Unified login/register tabs + Google OAuth button |
| `src/pages/SpotifyLink.tsx` | Prompt to link Spotify after initial login |

### Modified files

| File | Change |
|------|--------|
| `src/App.tsx` | Add `"init"`, `"auth"`, `"spotify-link"` screens; session check on mount; URL param handling for `?loggedin=true` and `?spotify_linked=true` |
| `src/pages/Landing.tsx` | Remove (replaced by `Auth.tsx`) |

---

## api.ts

Thin wrappers around `fetch`. All use `credentials: 'include'`.

```ts
checkSession(): Promise<User | null>
  → GET /users/me — returns User on success, null on 401

checkSpotifyStatus(): Promise<boolean>
  → GET /auth/spotify/status — returns { linked: boolean }

login(username: string, password: string): Promise<User>
  → POST /auth/login — throws on failure

register(username: string, displayName: string, password: string): Promise<User>
  → POST /auth/register — throws on failure
```

Google OAuth and Spotify link are redirects — no fetch wrappers needed (`window.location.href`).

---

## Auth.tsx

Unified screen with two tabs: **Login** and **Register**, plus a Google OAuth button.

**Login tab fields:** Username, Password
**Register tab fields:** Username, Display Name, Password

No email field — it's optional in the DB schema and omitted to keep the form minimal.

Error messages are shown inline (e.g. "Invalid username or password", "Username already taken").

**Google button:** `window.location.href = 'http://127.0.0.1:8080/auth/google'`

---

## SpotifyLink.tsx

Simple screen shown after login when Spotify is not yet linked. Explains that Spotify is needed for concert discovery. Has one button:

**"Connect Spotify"** → `window.location.href = 'http://127.0.0.1:8080/auth/spotify/link'`

---

## URL Param Handling (App.tsx useEffect)

On mount, after session check:

- `?loggedin=true` — set by Google OAuth callback. Call `checkSpotifyStatus()`, route to `"spotify-link"` or `"country"`. Clear the param from the URL.
- `?spotify_linked=true` — set by Spotify callback. Route to `"country"`. Clear the param.

---

## Init Flow (App.tsx useEffect on mount)

```
1. setScreen("init")  ← brief spinner
2. Check URL params first:
   a. ?spotify_linked=true → setScreen("country"), return
   b. ?loggedin=true → checkSpotifyStatus() → route, return
3. checkSession()
   a. null (no session) → setScreen("auth")
   b. User returned → checkSpotifyStatus()
       - linked   → setScreen("country")
       - unlinked → setScreen("spotify-link")
```

---

## Types

Add `User` type to match `/users/me` response:

```ts
interface User {
  id: string
  username: string | null
  display_name: string
  email: string | null
  spotify_linked: boolean
}
```

---

## Error Handling

- Network/server errors during login/register: show inline error message in the form
- Session check failure: treat as not logged in → show auth screen (don't show error)
- Spotify status check failure: default to `"spotify-link"` screen (safer than assuming it's linked)

---

## Verification

1. `cargo run` in `backend/` — server starts on port 8080
2. `npm run dev` in `frontend/` — Vite starts on port 5173
3. Test register → Spotify link → country → concerts
4. Test login (returning user with Spotify linked) → skips auth/link screens → goes straight to country
5. Test Google OAuth → Spotify link → country → concerts
6. Refresh while on country selection — should restore session and show country screen
7. `npm run build` — TypeScript check passes with no errors

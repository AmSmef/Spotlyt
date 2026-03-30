# Frontend Auth Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the old Spotify-only login flow with a full auth UI supporting username/password, Google OAuth, and Spotify account linking.

**Architecture:** Extend the existing `useState` screen machine in `App.tsx` with three new screens (`"init"`, `"auth"`, `"spotify-link"`). A new `api.ts` module centralises all fetch calls (all with `credentials: "include"`). Two new page components handle the auth and Spotify-link screens.

**Tech Stack:** React 18, TypeScript, Vite — no new dependencies.

---

## File Map

| Action | File | Responsibility |
|--------|------|----------------|
| Create | `frontend/src/api.ts` | Typed fetch helpers for all backend calls |
| Create | `frontend/src/pages/Auth.tsx` | Login/register tabs + Google OAuth button |
| Create | `frontend/src/pages/SpotifyLink.tsx` | Prompt to connect Spotify after login |
| Modify | `frontend/src/App.tsx` | New screen type, init flow, new screen renders |
| Modify | `frontend/src/App.css` | Styles for auth + spotify-link screens |
| Delete | `frontend/src/pages/Landing.tsx` | Replaced by Auth.tsx |

---

### Task 1: Create api.ts

**Files:**
- Create: `frontend/src/api.ts`

Backend response shapes confirmed from source:
- `GET /users/me` → `{ id, username: string|null, display_name, email: string|null, spotify_linked: bool }`
- `POST /auth/login` → `{ id, username: string|null, display_name, spotify_linked: bool }` + sets `session` cookie
- `POST /auth/register` → same shape as login + sets `session` cookie
- `GET /auth/spotify/status` → `{ linked: bool }`
- `POST /auth/register` body must be `{ username, password, display_name }` (snake_case)

- [ ] **Step 1: Create frontend/src/api.ts**

```typescript
const API = "http://127.0.0.1:8080";

export interface User {
  id: string;
  username: string | null;
  display_name: string;
  email?: string | null;
  spotify_linked: boolean;
}

export async function checkSession(): Promise<User | null> {
  try {
    const res = await fetch(`${API}/users/me`, { credentials: "include" });
    if (!res.ok) return null;
    return res.json();
  } catch {
    return null;
  }
}

export async function checkSpotifyStatus(): Promise<boolean> {
  try {
    const res = await fetch(`${API}/auth/spotify/status`, { credentials: "include" });
    if (!res.ok) return false;
    const data: { linked: boolean } = await res.json();
    return data.linked;
  } catch {
    return false;
  }
}

export async function login(username: string, password: string): Promise<User> {
  const res = await fetch(`${API}/auth/login`, {
    method: "POST",
    credentials: "include",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password }),
  });
  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || `Login failed (${res.status})`);
  }
  return res.json();
}

export async function register(
  username: string,
  displayName: string,
  password: string
): Promise<User> {
  const res = await fetch(`${API}/auth/register`, {
    method: "POST",
    credentials: "include",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, display_name: displayName, password }),
  });
  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || `Registration failed (${res.status})`);
  }
  return res.json();
}
```

- [ ] **Step 2: Commit**

```bash
cd /Users/adam/Documents/Spotlyt
git add frontend/src/api.ts
git commit -m "feat(frontend): add typed API helpers in api.ts"
```

---

### Task 2: Create Auth.tsx and its styles

**Files:**
- Create: `frontend/src/pages/Auth.tsx`
- Modify: `frontend/src/App.css`

- [ ] **Step 1: Create frontend/src/pages/Auth.tsx**

```tsx
import { useState } from "react";
import { login, register, User } from "../api";

type Tab = "login" | "register";

interface Props {
  onAuth: (user: User) => void;
}

export default function Auth({ onAuth }: Props) {
  const [tab, setTab] = useState<Tab>("login");
  const [username, setUsername] = useState("");
  const [displayName, setDisplayName] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  function switchTab(t: Tab) {
    setTab(t);
    setError("");
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError("");
    setLoading(true);
    try {
      const user =
        tab === "login"
          ? await login(username, password)
          : await register(username, displayName, password);
      onAuth(user);
    } catch (err: any) {
      setError(err.message ?? "Something went wrong");
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="screen screen-auth">
      <div className="auth-tabs">
        <button
          className={`auth-tab ${tab === "login" ? "auth-tab-active" : ""}`}
          onClick={() => switchTab("login")}
          type="button"
        >
          Sign in
        </button>
        <button
          className={`auth-tab ${tab === "register" ? "auth-tab-active" : ""}`}
          onClick={() => switchTab("register")}
          type="button"
        >
          Register
        </button>
      </div>

      <form className="auth-form" onSubmit={handleSubmit}>
        <div className="auth-field">
          <label className="auth-label">Username</label>
          <input
            className="auth-input"
            type="text"
            value={username}
            onChange={e => setUsername(e.target.value)}
            autoComplete="username"
            autoFocus
            required
          />
        </div>

        {tab === "register" && (
          <div className="auth-field">
            <label className="auth-label">Display name</label>
            <input
              className="auth-input"
              type="text"
              value={displayName}
              onChange={e => setDisplayName(e.target.value)}
              autoComplete="name"
              required
            />
          </div>
        )}

        <div className="auth-field">
          <label className="auth-label">Password</label>
          <input
            className="auth-input"
            type="password"
            value={password}
            onChange={e => setPassword(e.target.value)}
            autoComplete={tab === "login" ? "current-password" : "new-password"}
            required
          />
        </div>

        {error && <p className="auth-error">{error}</p>}

        <button className="btn-find" type="submit" disabled={loading}>
          {loading ? "…" : tab === "login" ? "Sign in" : "Create account"}
        </button>
      </form>

      <div className="auth-divider"><span>or</span></div>

      <button
        className="btn-google"
        type="button"
        onClick={() => { window.location.href = "http://127.0.0.1:8080/auth/google"; }}
      >
        <GoogleIcon />
        Continue with Google
      </button>
    </div>
  );
}

function GoogleIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" aria-hidden="true">
      <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
      <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
      <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l3.66-2.84z"/>
      <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
    </svg>
  );
}
```

- [ ] **Step 2: Append auth styles to frontend/src/App.css**

Add the following to the bottom of `App.css` (after the existing `@media` block):

```css
/* ── Auth ── */
.screen-auth {
  max-width: 400px;
}

.auth-tabs {
  display: flex;
  margin-bottom: 32px;
  border-bottom: 1px solid var(--border);
}

.auth-tab {
  background: none;
  border: none;
  font-family: var(--ff-mono);
  font-size: 11px;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: var(--muted);
  padding: 0 0 12px;
  margin-right: 24px;
  cursor: pointer;
  transition: color 0.12s;
  position: relative;
}

.auth-tab:hover { color: var(--text); }

.auth-tab-active { color: var(--accent); }

.auth-tab-active::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 0;
  right: 0;
  height: 1px;
  background: var(--accent);
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 18px;
  margin-bottom: 24px;
}

.auth-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.auth-label {
  font-family: var(--ff-mono);
  font-size: 10px;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  color: var(--muted);
}

.auth-input {
  background: var(--surface);
  border: 1px solid var(--border-hi);
  color: var(--text);
  font-family: var(--ff-body);
  font-size: 14px;
  padding: 11px 14px;
  border-radius: 3px;
  outline: none;
  transition: border-color 0.15s, box-shadow 0.15s;
}

.auth-input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-dim);
}

.auth-error {
  font-family: var(--ff-mono);
  font-size: 11px;
  color: var(--red);
  background: rgba(245, 92, 92, 0.08);
  border: 1px solid rgba(245, 92, 92, 0.2);
  border-radius: 3px;
  padding: 10px 14px;
  letter-spacing: 0.02em;
  line-height: 1.5;
}

.auth-divider {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
  color: var(--muted);
  font-family: var(--ff-mono);
  font-size: 10px;
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

.auth-divider::before,
.auth-divider::after {
  content: '';
  flex: 1;
  height: 1px;
  background: var(--border);
}

.btn-google {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  width: 100%;
  background: var(--surface);
  color: var(--text);
  border: 1px solid var(--border-hi);
  font-family: var(--ff-body);
  font-size: 14px;
  font-weight: 400;
  padding: 11px 20px;
  border-radius: 3px;
  cursor: pointer;
  transition: border-color 0.15s, background 0.15s;
}

.btn-google:hover {
  border-color: var(--muted);
  background: var(--faint);
}
```

- [ ] **Step 3: Commit**

```bash
cd /Users/adam/Documents/Spotlyt
git add frontend/src/pages/Auth.tsx frontend/src/App.css
git commit -m "feat(frontend): add Auth page with login/register tabs and Google OAuth"
```

---

### Task 3: Create SpotifyLink.tsx and its styles

**Files:**
- Create: `frontend/src/pages/SpotifyLink.tsx`
- Modify: `frontend/src/App.css`

- [ ] **Step 1: Create frontend/src/pages/SpotifyLink.tsx**

```tsx
import { SpotifyIcon } from "../components/Icons";

export default function SpotifyLink() {
  return (
    <div className="screen screen-spotify-link">
      <div className="landing-text">
        <h1 className="landing-headline">
          One more<br />
          <em>step.</em>
        </h1>
        <p className="landing-sub">
          Connect your Spotify account so we can find live concerts from your most-played artists.
        </p>
      </div>
      <button
        className="btn-spotify"
        onClick={() => { window.location.href = "http://127.0.0.1:8080/auth/spotify/link"; }}
      >
        <SpotifyIcon />
        Connect Spotify
      </button>
    </div>
  );
}
```

- [ ] **Step 2: Append spotify-link styles to frontend/src/App.css**

Add the following after the `.btn-google` block added in Task 2:

```css
/* ── Spotify Link ── */
.screen-spotify-link {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0;
}
```

- [ ] **Step 3: Commit**

```bash
cd /Users/adam/Documents/Spotlyt
git add frontend/src/pages/SpotifyLink.tsx frontend/src/App.css
git commit -m "feat(frontend): add SpotifyLink screen"
```

---

### Task 4: Update App.tsx and remove Landing.tsx

**Files:**
- Modify: `frontend/src/App.tsx`
- Delete: `frontend/src/pages/Landing.tsx`

Key changes:
- New `Screen` type: `"init" | "auth" | "spotify-link" | "country" | "loading" | "results" | "error"`
- Add `user` state
- Replace `useEffect` URL param check with full init flow (session check + URL params)
- Add `handleAuth` callback called by `Auth` after successful login/register
- Fix `handleFetch` to include `credentials: "include"` (concerts endpoint now requires auth)
- Remove `handleLogin` function and `Landing` import

- [ ] **Step 1: Replace frontend/src/App.tsx with the following**

```tsx
import { useState, useEffect } from "react";
import "./App.css";
import Auth from "./pages/Auth";
import SpotifyLink from "./pages/SpotifyLink";
import CountrySelection from "./pages/CountrySelection";
import Loading from "./pages/Loading";
import Home from "./pages/Home";
import ErrorPage from "./pages/Error";
import { checkSession, checkSpotifyStatus, User } from "./api";

const API = "http://127.0.0.1:8080";

interface Concert {
  artist_name: string;
  event_name: string;
  venue: string;
  city: string;
  date: string;
}

type Screen = "init" | "auth" | "spotify-link" | "country" | "loading" | "results" | "error";

export default function App() {
  const [screen, setScreen] = useState<Screen>("init");
  const [user, setUser] = useState<User | null>(null);
  const [country, setCountry] = useState("");
  const [concerts, setConcerts] = useState<Concert[]>([]);
  const [errorMsg, setErrorMsg] = useState("");
  const [loadingDots, setLoadingDots] = useState("");

  useEffect(() => {
    async function init() {
      const params = new URLSearchParams(window.location.search);
      window.history.replaceState({}, "", "/");

      if (params.get("spotify_linked") === "true") {
        setScreen("country");
        return;
      }

      if (params.get("loggedin") === "true") {
        const linked = await checkSpotifyStatus();
        setScreen(linked ? "country" : "spotify-link");
        return;
      }

      const sessionUser = await checkSession();
      if (!sessionUser) {
        setScreen("auth");
        return;
      }
      setUser(sessionUser);
      setScreen(sessionUser.spotify_linked ? "country" : "spotify-link");
    }
    init();
  }, []);

  useEffect(() => {
    if (screen !== "loading") return;
    const interval = setInterval(() => {
      setLoadingDots(d => d.length >= 3 ? "" : d + ".");
    }, 400);
    return () => clearInterval(interval);
  }, [screen]);

  function handleAuth(authUser: User) {
    setUser(authUser);
    setScreen(authUser.spotify_linked ? "country" : "spotify-link");
  }

  async function handleFetch() {
    const code = country.trim().toUpperCase();
    if (code.length !== 2) return;
    setScreen("loading");
    try {
      const res = await fetch(`${API}/concerts?country=${code}`, {
        credentials: "include",
      });
      if (!res.ok) {
        const text = await res.text();
        throw new Error(text || `Server error ${res.status}`);
      }
      const data: Concert[] = await res.json();
      setConcerts(data);
      setScreen("results");
    } catch (e: any) {
      setErrorMsg(e.message ?? "Unknown error");
      setScreen("error");
    }
  }

  return (
    <div className="app">
      <header>
        <div className="wordmark">
          <span className="wordmark-spot">Spot</span>
          <span className="wordmark-lyt">lyt</span>
        </div>
        <div className="header-rule" />
      </header>

      <main>
        {screen === "init" && null}
        {screen === "auth" && <Auth onAuth={handleAuth} />}
        {screen === "spotify-link" && <SpotifyLink />}
        {screen === "country" && (
          <CountrySelection
            country={country}
            onCountryChange={setCountry}
            onFetch={handleFetch}
          />
        )}
        {screen === "loading" && <Loading dots={loadingDots} />}
        {screen === "results" && (
          <Home
            concerts={concerts}
            country={country}
            onBack={() => { setCountry(""); setScreen("country"); }}
          />
        )}
        {screen === "error" && (
          <ErrorPage
            message={errorMsg}
            onBack={() => setScreen("country")}
          />
        )}
      </main>

      <footer>
        <span>Spotlyt</span>
        <span>Powered by Spotify & Ticketmaster</span>
      </footer>
    </div>
  );
}
```

- [ ] **Step 2: Delete frontend/src/pages/Landing.tsx**

```bash
rm /Users/adam/Documents/Spotlyt/frontend/src/pages/Landing.tsx
```

- [ ] **Step 3: Verify TypeScript compiles cleanly**

Run from `frontend/`:

```bash
cd /Users/adam/Documents/Spotlyt/frontend && npm run build
```

Expected output: build succeeds with no TypeScript errors. If errors appear, fix them before committing.

- [ ] **Step 4: Commit**

```bash
cd /Users/adam/Documents/Spotlyt
git add frontend/src/App.tsx
git rm frontend/src/pages/Landing.tsx
git commit -m "feat(frontend): replace landing screen with multi-provider auth flow"
```

---

## Verification

Manual test checklist (both `cargo run` in `backend/` and `npm run dev` in `frontend/` must be running):

1. **New user — password auth**: Open app → auth screen shown → Register tab → fill username/display name/password → submit → Spotify link screen shown → click Connect Spotify → redirected to Spotify → return to app → country screen shown.

2. **Returning user with Spotify linked**: Refresh page → session check finds existing session → Spotify linked → country screen shown directly (no auth or link screens).

3. **Returning user without Spotify linked**: If a session exists but Spotify isn't linked → spotify-link screen shown.

4. **Google OAuth**: Auth screen → Continue with Google → Google login → redirected back with `?loggedin=true` → Spotify status checked → routed to Spotify link or country screen.

5. **Wrong password**: Login tab → wrong password → inline error "Invalid username or password" shown.

6. **Duplicate username**: Register tab → existing username → inline error "Username or email already taken".

7. **Concert fetch**: Reach country screen → enter country code → concerts load correctly (session cookie is sent with the request).

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
    id                        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username                  TEXT UNIQUE,
    password_hash             TEXT,
    email                     TEXT UNIQUE,
    display_name              TEXT NOT NULL,
    google_id                 TEXT UNIQUE,
    spotify_refresh_token     TEXT,
    spotify_access_token      TEXT,
    spotify_token_expires_at  TIMESTAMPTZ,
    created_at                TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- must have at least one auth method
    CONSTRAINT must_have_auth CHECK (
        password_hash IS NOT NULL OR google_id IS NOT NULL
    )
);

CREATE TABLE sessions (
    token       TEXT PRIMARY KEY,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at  TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '30 days'
);

CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);

-- Ties a Spotify OAuth CSRF token to the user who initiated the link flow
CREATE TABLE spotify_oauth_state (
    csrf_token  TEXT PRIMARY KEY,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at  TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '10 minutes'
);

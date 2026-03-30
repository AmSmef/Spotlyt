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

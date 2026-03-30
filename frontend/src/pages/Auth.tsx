import { useState } from "react";
import { login, register, User } from "../api";
import { GoogleIcon } from "../components/Icons";

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
    setUsername("");
    setDisplayName("");
    setPassword("");
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
    } catch (err) {
      setError(err instanceof Error ? err.message : "Something went wrong");
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
          <label className="auth-label" htmlFor="username">Username</label>
          <input
            id="username"
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
            <label className="auth-label" htmlFor="display-name">Display name</label>
            <input
              id="display-name"
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
          <label className="auth-label" htmlFor="password">Password</label>
          <input
            id="password"
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


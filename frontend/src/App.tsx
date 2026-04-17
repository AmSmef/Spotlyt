import { useState, useEffect } from "react";
import "./App.css";
import Auth from "./pages/Auth";
import SpotifyLink from "./pages/SpotifyLink";
import CountrySelection from "./pages/CountrySelection";
import Loading from "./pages/Loading";
import Home from "./pages/Home";
import ErrorPage from "./pages/Error";
import { checkSession, User } from "./api";

const API = "http://127.0.0.1:8080";

interface Concert {
  artist_names: string[];
  event_name: string;
  venue: string;
  city: string;
  date: string;
  image_url: string | null;
}

type Screen = "init" | "auth" | "spotify-link" | "country" | "loading" | "results" | "error";

export default function App() {
  const [screen, setScreen] = useState<Screen>("init");
  // user is stored for future display-name rendering
  const [user, setUser] = useState<User | null>(null);
  void user; // referenced here until display-name rendering is wired up
  const [country, setCountry] = useState("");
  const [concerts, setConcerts] = useState<Concert[]>([]);
  const [errorMsg, setErrorMsg] = useState("");
  const [loadingDots, setLoadingDots] = useState("");

  useEffect(() => {
    async function init() {
      const params = new URLSearchParams(window.location.search);
      window.history.replaceState({}, "", "/");

      if (params.get("spotify_linked") === "true") {
        const sessionUser = await checkSession();
        if (!sessionUser) { setScreen("auth"); return; }
        setUser(sessionUser);
        setScreen("country");
        return;
      }

      if (params.get("loggedin") === "true") {
        const sessionUser = await checkSession();
        if (!sessionUser) { setScreen("auth"); return; }
        setUser(sessionUser);
        setScreen(sessionUser.spotify_linked ? "country" : "spotify-link");
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
    setErrorMsg("");
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
    } catch (e) {
      setErrorMsg(e instanceof Error ? e.message : "Unknown error");
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
        {screen === "init" && <Loading dots="" />}
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

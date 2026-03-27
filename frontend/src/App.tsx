import { useState, useEffect } from "react";
import "./App.css";
import Landing from "./pages/Landing";
import CountrySelection from "./pages/CountrySelection";
import Loading from "./pages/Loading";
import Home from "./pages/Home";
import ErrorPage from "./pages/Error";

const API = "http://127.0.0.1:8080";

interface Concert {
  artist_name: string;
  event_name: string;
  venue: string;
  city: string;
  date: string;
}

type Screen = "landing" | "country" | "loading" | "results" | "error";

export default function App() {
  const [screen, setScreen] = useState<Screen>("landing");
  const [country, setCountry] = useState("");
  const [concerts, setConcerts] = useState<Concert[]>([]);
  const [errorMsg, setErrorMsg] = useState("");
  const [loadingDots, setLoadingDots] = useState("");

  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    if (params.get("loggedin") === "true") {
      window.history.replaceState({}, "", "/");
      setScreen("country");
    }
  }, []);

  useEffect(() => {
    if (screen !== "loading") return;
    const interval = setInterval(() => {
      setLoadingDots(d => d.length >= 3 ? "" : d + ".");
    }, 400);
    return () => clearInterval(interval);
  }, [screen]);

  function handleLogin() {
    window.location.href = `${API}/auth/login`;
  }

  async function handleFetch() {
    const code = country.trim().toUpperCase();
    if (code.length !== 2) return;
    setScreen("loading");
    try {
      const res = await fetch(`${API}/concerts?country=${code}`);
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
        {screen === "landing" && <Landing onLogin={handleLogin} />}
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

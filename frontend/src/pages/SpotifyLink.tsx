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

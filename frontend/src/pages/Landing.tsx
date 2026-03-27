import { SpotifyIcon } from "../components/Icons";

interface Props {
  onLogin: () => void;
}

export default function Landing({ onLogin }: Props) {
  return (
    <div className="screen screen-landing">
      <div className="landing-text">
        <h1 className="landing-headline">
          Your taste,<br />
          <em>on stage.</em>
        </h1>
        <p className="landing-sub">
          Connect Spotify and we'll find live concerts from your most-played artists — filtered by country, sorted by date.
        </p>
      </div>
      <button className="btn-spotify" onClick={onLogin}>
        <SpotifyIcon />
        Continue with Spotify
      </button>
      <p className="landing-note">Reads your top artists. Nothing else.</p>
    </div>
  );
}
